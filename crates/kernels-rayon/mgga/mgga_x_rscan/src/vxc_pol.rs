//! MGGA_X_RSCAN vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rscan.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_rscan_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_alphar: f64,
    param_c2: f64,
    param_d: f64,
    param_k1: f64,
    param_taur: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alphar = f64x8::splat(param_alphar);
    let param_c2 = f64x8::splat(param_c2);
    let param_d = f64x8::splat(param_d);
    let param_k1 = f64x8::splat(param_k1);
    let param_taur = f64x8::splat(param_taur);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_vlapl_0 = V_ZERO;
        let mut acc_vlapl_1 = V_ZERO;
        let mut acc_vtau_0 = V_ZERO;
        let mut acc_vtau_1 = V_ZERO;
        {
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * zeta_threshold;
            let t24 = (simd::cbrt(t20));
            let t26 = ((t21).select(t23, t24 * t20));
            let t27 = t6 * t26;
            let t28 = (simd::cbrt(t7));
            let t29 = f64x8::splat(M_CBRT6);
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t29 * t33;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t38 = t37 * t35;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = v_sigma0 * t39;
            let t41 = t34 * t40;
            let t45 = f64x8::splat(100.0) / f64x8::splat(6561.0) / param_k1 - f64x8::splat(73.0) / f64x8::splat(648.0);
            let t46 = t29 * t29;
            let t47 = t45 * t46;
            let t48 = t31 * t30;
            let t49 = f64x8::splat(1.0) / t48;
            let t50 = t47 * t49;
            let t51 = v_sigma0 * v_sigma0;
            let t52 = t35 * t35;
            let t53 = t52 * v_rho0;
            let t55 = f64x8::splat(1.0) / t36 / t53;
            let t56 = t51 * t55;
            let t57 = t45 * t29;
            let t58 = t33 * v_sigma0;
            let t59 = t58 * t39;
            let t62 = (simd::exp(-f64x8::splat(27.0) / f64x8::splat(80.0) * t57 * t59));
            let t66 = ((f64x8::splat(146.0)).sqrt());
            let t67 = t66 * t29;
            let t70 = t20 * t20;
            let t71 = t70 * t70;
            let t72 = t71 * t20;
            let t73 = t7 * t7;
            let t74 = t73 * t73;
            let t75 = t74 * t7;
            let t76 = t72 * t75;
            let t77 = t37 * v_rho0;
            let t78 = f64x8::splat(1.0) / t77;
            let t81 = v_tau0 * t78 - t40 / f64x8::splat(8.0);
            let t82 = (f64x8::splat(0.0)).simd_lt(t81);
            let t83 = ((t82).select(t81, f64x8::splat(0.0)));
            let t84 = t83 * t83;
            let t85 = t84 * t83;
            let t86 = f64x8::splat(M_CBRT2);
            let t87 = t20 * t7;
            let t88 = (simd::cbrt(t87));
            let t89 = t88 * t88;
            let t92 = t46 * t32;
            let t95 = param_taur / f64x8::splat(2.0);
            let t96 = f64x8::splat(3.0) / f64x8::splat(40.0) * t86 * t89 * t87 * t92 + t95;
            let t97 = t96 * t96;
            let t98 = t97 * t96;
            let t99 = f64x8::splat(1.0) / t98;
            let t100 = t85 * t99;
            let t101 = t86 * t86;
            let t102 = t70 * t20;
            let t103 = t73 * t7;
            let t104 = t102 * t103;
            let t105 = t88 * t104;
            let t106 = t101 * t105;
            let t107 = f64x8::splat(1.0) / t97;
            let t108 = t84 * t107;
            let t111 = t106 * t108 / f64x8::splat(16.0) + param_alphar;
            let t112 = f64x8::splat(1.0) / t111;
            let t113 = t100 * t112;
            let t115 = t76 * t113 / f64x8::splat(32.0);
            let t116 = f64x8::splat(1.0) - t115;
            let t118 = t116 * t116;
            let t120 = (simd::exp(-t118 / f64x8::splat(2.0)));
            let t123 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t67 * t59 + t66 * t116 * t120 / f64x8::splat(100.0);
            let t124 = t123 * t123;
            let t125 = param_k1 + f64x8::splat(5.0) / f64x8::splat(972.0) * t41 + t50 * t56 * t62 / f64x8::splat(576.0) + t124;
            let t130 = f64x8::splat(1.0) + param_k1 * (f64x8::splat(1.0) - param_k1 / t125);
            let t131 = (t115).simd_le(f64x8::splat(2.5));
            let t132 = (f64x8::splat(2.5)).simd_lt(t115);
            let t133 = ((t132).select(f64x8::splat(2.5), t115));
            let t135 = t133 * t133;
            let t137 = t135 * t133;
            let t139 = t135 * t135;
            let t141 = t139 * t133;
            let t143 = t139 * t135;
            let t148 = ((t132).select(t115, f64x8::splat(2.5)));
            let t149 = f64x8::splat(1.0) - t148;
            let t152 = (simd::exp(param_c2 / t149));
            let t154 = ((t131).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t133 - f64x8::splat(0.4445555) * t135 - f64x8::splat(0.663086601049) * t137 + f64x8::splat(1.45129704449) * t139 - f64x8::splat(0.887998041597) * t141 + f64x8::splat(0.234528941479) * t143 - f64x8::splat(0.023185843322) * t139 * t137, -param_d * t152));
            let t155 = f64x8::splat(1.0) - t154;
            let t158 = t130 * t155 + f64x8::splat(1.174) * t154;
            let t159 = t28 * t158;
            let t160 = ((f64x8::splat(3.0)).sqrt());
            let t161 = f64x8::splat(1.0) / t31;
            let t162 = t46 * t161;
            let t163 = ((v_sigma0).sqrt());
            let t164 = t36 * v_rho0;
            let t165 = f64x8::splat(1.0) / t164;
            let t167 = t162 * t163 * t165;
            let t168 = ((t167).sqrt());
            let t172 = (simd::exp(-f64x8::splat(9.8958) * t160 / t168));
            let t173 = f64x8::splat(1.0) - t172;
            let t174 = t159 * t173;
            let t177 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t174));
            let t178 = (v_rho1).simd_le(dens_threshold);
            let t179 = -t17;
            let t181 = ((t15).select(t12, (t11).select(t16, t179 * t8)));
            let t182 = f64x8::splat(1.0) + t181;
            let t183 = (t182).simd_le(zeta_threshold);
            let t184 = (simd::cbrt(t182));
            let t186 = ((t183).select(t23, t184 * t182));
            let t187 = t6 * t186;
            let t188 = v_rho1 * v_rho1;
            let t189 = (simd::cbrt(v_rho1));
            let t190 = t189 * t189;
            let t191 = t190 * t188;
            let t192 = f64x8::splat(1.0) / t191;
            let t193 = v_sigma2 * t192;
            let t194 = t34 * t193;
            let t196 = v_sigma2 * v_sigma2;
            let t197 = t188 * t188;
            let t198 = t197 * v_rho1;
            let t200 = f64x8::splat(1.0) / t189 / t198;
            let t201 = t196 * t200;
            let t202 = t33 * v_sigma2;
            let t203 = t202 * t192;
            let t206 = (simd::exp(-f64x8::splat(27.0) / f64x8::splat(80.0) * t57 * t203));
            let t212 = t182 * t182;
            let t213 = t212 * t212;
            let t214 = t213 * t182;
            let t215 = t214 * t75;
            let t216 = t190 * v_rho1;
            let t217 = f64x8::splat(1.0) / t216;
            let t220 = v_tau1 * t217 - t193 / f64x8::splat(8.0);
            let t221 = (f64x8::splat(0.0)).simd_lt(t220);
            let t222 = ((t221).select(t220, f64x8::splat(0.0)));
            let t223 = t222 * t222;
            let t224 = t223 * t222;
            let t225 = t182 * t7;
            let t226 = (simd::cbrt(t225));
            let t227 = t226 * t226;
            let t232 = f64x8::splat(3.0) / f64x8::splat(40.0) * t86 * t227 * t225 * t92 + t95;
            let t233 = t232 * t232;
            let t234 = t233 * t232;
            let t235 = f64x8::splat(1.0) / t234;
            let t236 = t224 * t235;
            let t237 = t212 * t182;
            let t238 = t237 * t103;
            let t239 = t226 * t238;
            let t240 = t101 * t239;
            let t241 = f64x8::splat(1.0) / t233;
            let t242 = t223 * t241;
            let t245 = t240 * t242 / f64x8::splat(16.0) + param_alphar;
            let t246 = f64x8::splat(1.0) / t245;
            let t247 = t236 * t246;
            let t249 = t215 * t247 / f64x8::splat(32.0);
            let t250 = f64x8::splat(1.0) - t249;
            let t252 = t250 * t250;
            let t254 = (simd::exp(-t252 / f64x8::splat(2.0)));
            let t257 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t67 * t203 + t66 * t250 * t254 / f64x8::splat(100.0);
            let t258 = t257 * t257;
            let t259 = param_k1 + f64x8::splat(5.0) / f64x8::splat(972.0) * t194 + t50 * t201 * t206 / f64x8::splat(576.0) + t258;
            let t264 = f64x8::splat(1.0) + param_k1 * (f64x8::splat(1.0) - param_k1 / t259);
            let t265 = (t249).simd_le(f64x8::splat(2.5));
            let t266 = (f64x8::splat(2.5)).simd_lt(t249);
            let t267 = ((t266).select(f64x8::splat(2.5), t249));
            let t269 = t267 * t267;
            let t271 = t269 * t267;
            let t273 = t269 * t269;
            let t275 = t273 * t267;
            let t277 = t273 * t269;
            let t282 = ((t266).select(t249, f64x8::splat(2.5)));
            let t283 = f64x8::splat(1.0) - t282;
            let t286 = (simd::exp(param_c2 / t283));
            let t288 = ((t265).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t267 - f64x8::splat(0.4445555) * t269 - f64x8::splat(0.663086601049) * t271 + f64x8::splat(1.45129704449) * t273 - f64x8::splat(0.887998041597) * t275 + f64x8::splat(0.234528941479) * t277 - f64x8::splat(0.023185843322) * t273 * t271, -param_d * t286));
            let t289 = f64x8::splat(1.0) - t288;
            let t292 = t264 * t289 + f64x8::splat(1.174) * t288;
            let t293 = t28 * t292;
            let t294 = ((v_sigma2).sqrt());
            let t295 = t189 * v_rho1;
            let t296 = f64x8::splat(1.0) / t295;
            let t298 = t162 * t294 * t296;
            let t299 = ((t298).sqrt());
            let t303 = (simd::exp(-f64x8::splat(9.8958) * t160 / t299));
            let t304 = f64x8::splat(1.0) - t303;
            let t305 = t293 * t304;
            let t308 = ((t178).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t187 * t305));
            let tzk0 = t177 + t308;
            acc_zk = tzk0;
            let t309 = f64x8::splat(1.0) / t73;
            let t310 = t17 * t309;
            let t312 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t310)));
            let t315 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t312));
            let t316 = t6 * t315;
            let t319 = t28 * t28;
            let t320 = f64x8::splat(1.0) / t319;
            let t321 = t320 * t158;
            let t322 = t321 * t173;
            let t324 = t27 * t322 / f64x8::splat(8.0);
            let t325 = param_k1 * param_k1;
            let t326 = t125 * t125;
            let t328 = t325 / t326;
            let t329 = t35 * v_rho0;
            let t331 = f64x8::splat(1.0) / t37 / t329;
            let t332 = v_sigma0 * t331;
            let t335 = t52 * t35;
            let t337 = f64x8::splat(1.0) / t36 / t335;
            let t342 = t45 * t45;
            let t343 = t30 * t30;
            let t344 = f64x8::splat(1.0) / t343;
            let t345 = t342 * t344;
            let t346 = t51 * v_sigma0;
            let t347 = t52 * t52;
            let t348 = t347 * v_rho0;
            let t349 = f64x8::splat(1.0) / t348;
            let t357 = t71 * t75;
            let t358 = t357 * t85;
            let t359 = t99 * t112;
            let t360 = t359 * t312;
            let t363 = t72 * t74;
            let t365 = f64x8::splat(5.0) / f64x8::splat(32.0) * t363 * t113;
            let t366 = t76 * t84;
            let t371 = ((t82).select(-f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau0 * t39 + t332 / f64x8::splat(3.0), f64x8::splat(0.0)));
            let t372 = t359 * t371;
            let t375 = t97 * t97;
            let t376 = f64x8::splat(1.0) / t375;
            let t377 = t85 * t376;
            let t378 = t377 * t112;
            let t379 = t76 * t378;
            let t380 = t86 * t89;
            let t382 = t312 * t7 + t19 + f64x8::splat(1.0);
            let t383 = t92 * t382;
            let t384 = t380 * t383;
            let t387 = t76 * t85;
            let t388 = t111 * t111;
            let t389 = f64x8::splat(1.0) / t388;
            let t390 = t99 * t389;
            let t391 = t70 * t73;
            let t392 = t88 * t391;
            let t393 = t101 * t392;
            let t397 = t83 * t107;
            let t401 = t71 * t74;
            let t402 = t401 * t84;
            let t403 = t99 * t46;
            let t404 = t32 * t382;
            let t405 = t403 * t404;
            let t408 = f64x8::splat(5.0) / f64x8::splat(24.0) * t393 * t108 * t382 + t106 * t397 * t371 / f64x8::splat(8.0) - t402 * t405 / f64x8::splat(32.0);
            let t409 = t390 * t408;
            let t412 = -f64x8::splat(5.0) / f64x8::splat(32.0) * t358 * t360 - t365 - f64x8::splat(3.0) / f64x8::splat(32.0) * t366 * t372 + f64x8::splat(3.0) / f64x8::splat(256.0) * t379 * t384 + t387 * t409 / f64x8::splat(32.0);
            let t416 = t66 * t118;
            let t417 = t412 * t120;
            let t420 = -f64x8::splat(7.0) / f64x8::splat(4860.0) * t67 * t58 * t331 + t66 * t412 * t120 / f64x8::splat(100.0) - t416 * t417 / f64x8::splat(100.0);
            let t423 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t34 * t332 - t50 * t51 * t337 * t62 / f64x8::splat(108.0) + f64x8::splat(3.0) / f64x8::splat(320.0) * t345 * t346 * t349 * t62 + f64x8::splat(2.0) * t123 * t420;
            let t424 = t423 * t155;
            let t426 = -t412;
            let t427 = ((t132).select(f64x8::splat(0.0), t426));
            let t429 = t133 * t427;
            let t431 = t135 * t427;
            let t433 = t137 * t427;
            let t435 = t139 * t427;
            let t437 = t141 * t427;
            let t442 = param_d * param_c2;
            let t443 = t149 * t149;
            let t444 = f64x8::splat(1.0) / t443;
            let t445 = ((t132).select(t426, f64x8::splat(0.0)));
            let t449 = ((t131).select(-f64x8::splat(0.667) * t427 - f64x8::splat(0.889111) * t429 - f64x8::splat(1.989259803147) * t431 + f64x8::splat(5.80518817796) * t433 - f64x8::splat(4.439990207985) * t435 + f64x8::splat(1.407173648874) * t437 - f64x8::splat(0.162300903254) * t143 * t427, -t442 * t444 * t445 * t152));
            let t452 = t328 * t424 - t130 * t449 + f64x8::splat(1.174) * t449;
            let t453 = t28 * t452;
            let t454 = t453 * t173;
            let t457 = (simd::pow(f64x8::splat(3.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t458 = t457 * t457;
            let t459 = t458 * t458;
            let t460 = t459 * t457;
            let t461 = t460 * t26;
            let t463 = f64x8::splat(1.0) / t168 / t167;
            let t464 = t159 * t463;
            let t465 = t461 * t464;
            let t467 = f64x8::splat(1.0) / t36 / t35;
            let t470 = t162 * t163 * t467 * t172;
            let t474 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t316 * t174 - t324 - f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t454 - f64x8::splat(1.6891736332904388) * t465 * t470));
            let t475 = t179 * t309;
            let t477 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t475)));
            let t480 = ((t183).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t184 * t477));
            let t481 = t6 * t480;
            let t484 = t320 * t292;
            let t485 = t484 * t304;
            let t487 = t187 * t485 / f64x8::splat(8.0);
            let t488 = t259 * t259;
            let t490 = t325 / t488;
            let t491 = t213 * t75;
            let t492 = t491 * t224;
            let t493 = t235 * t246;
            let t494 = t493 * t477;
            let t497 = t214 * t74;
            let t499 = f64x8::splat(5.0) / f64x8::splat(32.0) * t497 * t247;
            let t500 = t233 * t233;
            let t501 = f64x8::splat(1.0) / t500;
            let t502 = t224 * t501;
            let t503 = t502 * t246;
            let t504 = t215 * t503;
            let t505 = t86 * t227;
            let t507 = t477 * t7 + t181 + f64x8::splat(1.0);
            let t508 = t92 * t507;
            let t509 = t505 * t508;
            let t512 = t215 * t224;
            let t513 = t245 * t245;
            let t514 = f64x8::splat(1.0) / t513;
            let t515 = t235 * t514;
            let t516 = t212 * t73;
            let t517 = t226 * t516;
            let t518 = t101 * t517;
            let t522 = t213 * t74;
            let t523 = t522 * t223;
            let t524 = t235 * t46;
            let t525 = t32 * t507;
            let t526 = t524 * t525;
            let t529 = f64x8::splat(5.0) / f64x8::splat(24.0) * t518 * t242 * t507 - t523 * t526 / f64x8::splat(32.0);
            let t530 = t515 * t529;
            let t533 = -f64x8::splat(5.0) / f64x8::splat(32.0) * t492 * t494 - t499 + f64x8::splat(3.0) / f64x8::splat(256.0) * t504 * t509 + t512 * t530 / f64x8::splat(32.0);
            let t536 = t66 * t252;
            let t537 = t533 * t254;
            let t540 = t66 * t533 * t254 / f64x8::splat(100.0) - t536 * t537 / f64x8::splat(100.0);
            let t541 = t257 * t540;
            let t542 = t541 * t289;
            let t545 = -t533;
            let t546 = ((t266).select(f64x8::splat(0.0), t545));
            let t548 = t267 * t546;
            let t550 = t269 * t546;
            let t552 = t271 * t546;
            let t554 = t273 * t546;
            let t556 = t275 * t546;
            let t561 = t283 * t283;
            let t562 = f64x8::splat(1.0) / t561;
            let t563 = ((t266).select(t545, f64x8::splat(0.0)));
            let t567 = ((t265).select(-f64x8::splat(0.667) * t546 - f64x8::splat(0.889111) * t548 - f64x8::splat(1.989259803147) * t550 + f64x8::splat(5.80518817796) * t552 - f64x8::splat(4.439990207985) * t554 + f64x8::splat(1.407173648874) * t556 - f64x8::splat(0.162300903254) * t277 * t546, -t442 * t562 * t563 * t286));
            let t570 = f64x8::splat(2.0) * t490 * t542 - t264 * t567 + f64x8::splat(1.174) * t567;
            let t571 = t28 * t570;
            let t572 = t571 * t304;
            let t576 = ((t178).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t481 * t305 - t487 - f64x8::splat(3.0) / f64x8::splat(8.0) * t187 * t572));
            let tvrho0 = t177 + t308 + t7 * (t474 + t576);
            acc_vrho_0 = tvrho0;
            let t580 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t310)));
            let t583 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t580));
            let t584 = t6 * t583;
            let t587 = t359 * t580;
            let t591 = t580 * t7 + t19 + f64x8::splat(1.0);
            let t592 = t92 * t591;
            let t593 = t380 * t592;
            let t599 = t32 * t591;
            let t600 = t403 * t599;
            let t603 = f64x8::splat(5.0) / f64x8::splat(24.0) * t393 * t108 * t591 - t402 * t600 / f64x8::splat(32.0);
            let t604 = t390 * t603;
            let t607 = -f64x8::splat(5.0) / f64x8::splat(32.0) * t358 * t587 - t365 + f64x8::splat(3.0) / f64x8::splat(256.0) * t379 * t593 + t387 * t604 / f64x8::splat(32.0);
            let t608 = t66 * t607;
            let t610 = t607 * t120;
            let t613 = t608 * t120 / f64x8::splat(100.0) - t416 * t610 / f64x8::splat(100.0);
            let t614 = t123 * t613;
            let t615 = t614 * t155;
            let t618 = -t607;
            let t619 = ((t132).select(f64x8::splat(0.0), t618));
            let t621 = t133 * t619;
            let t623 = t135 * t619;
            let t625 = t137 * t619;
            let t627 = t139 * t619;
            let t629 = t141 * t619;
            let t634 = ((t132).select(t618, f64x8::splat(0.0)));
            let t638 = ((t131).select(-f64x8::splat(0.667) * t619 - f64x8::splat(0.889111) * t621 - f64x8::splat(1.989259803147) * t623 + f64x8::splat(5.80518817796) * t625 - f64x8::splat(4.439990207985) * t627 + f64x8::splat(1.407173648874) * t629 - f64x8::splat(0.162300903254) * t143 * t619, -t442 * t444 * t634 * t152));
            let t641 = f64x8::splat(2.0) * t328 * t615 - t130 * t638 + f64x8::splat(1.174) * t638;
            let t642 = t28 * t641;
            let t643 = t642 * t173;
            let t647 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t584 * t174 - t324 - f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t643));
            let t649 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t475)));
            let t652 = ((t183).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t184 * t649));
            let t653 = t6 * t652;
            let t656 = t188 * v_rho1;
            let t658 = f64x8::splat(1.0) / t190 / t656;
            let t659 = v_sigma2 * t658;
            let t662 = t197 * t188;
            let t664 = f64x8::splat(1.0) / t189 / t662;
            let t669 = t196 * v_sigma2;
            let t670 = t197 * t197;
            let t671 = t670 * v_rho1;
            let t672 = f64x8::splat(1.0) / t671;
            let t680 = t493 * t649;
            let t683 = t215 * t223;
            let t688 = ((t221).select(-f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau1 * t192 + t659 / f64x8::splat(3.0), f64x8::splat(0.0)));
            let t689 = t493 * t688;
            let t693 = t649 * t7 + t181 + f64x8::splat(1.0);
            let t694 = t92 * t693;
            let t695 = t505 * t694;
            let t701 = t222 * t241;
            let t705 = t32 * t693;
            let t706 = t524 * t705;
            let t709 = f64x8::splat(5.0) / f64x8::splat(24.0) * t518 * t242 * t693 + t240 * t701 * t688 / f64x8::splat(8.0) - t523 * t706 / f64x8::splat(32.0);
            let t710 = t515 * t709;
            let t713 = -f64x8::splat(5.0) / f64x8::splat(32.0) * t492 * t680 - t499 - f64x8::splat(3.0) / f64x8::splat(32.0) * t683 * t689 + f64x8::splat(3.0) / f64x8::splat(256.0) * t504 * t695 + t512 * t710 / f64x8::splat(32.0);
            let t714 = t66 * t713;
            let t717 = t713 * t254;
            let t720 = -f64x8::splat(7.0) / f64x8::splat(4860.0) * t67 * t202 * t658 + t714 * t254 / f64x8::splat(100.0) - t536 * t717 / f64x8::splat(100.0);
            let t723 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t34 * t659 - t50 * t196 * t664 * t206 / f64x8::splat(108.0) + f64x8::splat(3.0) / f64x8::splat(320.0) * t345 * t669 * t672 * t206 + f64x8::splat(2.0) * t257 * t720;
            let t724 = t723 * t289;
            let t726 = -t713;
            let t727 = ((t266).select(f64x8::splat(0.0), t726));
            let t729 = t267 * t727;
            let t731 = t269 * t727;
            let t733 = t271 * t727;
            let t735 = t273 * t727;
            let t737 = t275 * t727;
            let t742 = ((t266).select(t726, f64x8::splat(0.0)));
            let t746 = ((t265).select(-f64x8::splat(0.667) * t727 - f64x8::splat(0.889111) * t729 - f64x8::splat(1.989259803147) * t731 + f64x8::splat(5.80518817796) * t733 - f64x8::splat(4.439990207985) * t735 + f64x8::splat(1.407173648874) * t737 - f64x8::splat(0.162300903254) * t277 * t727, -t442 * t562 * t742 * t286));
            let t749 = t490 * t724 - t264 * t746 + f64x8::splat(1.174) * t746;
            let t750 = t28 * t749;
            let t751 = t750 * t304;
            let t754 = t460 * t186;
            let t756 = f64x8::splat(1.0) / t299 / t298;
            let t757 = t293 * t756;
            let t758 = t754 * t757;
            let t760 = f64x8::splat(1.0) / t189 / t188;
            let t763 = t162 * t294 * t760 * t303;
            let t767 = ((t178).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t653 * t305 - t487 - f64x8::splat(3.0) / f64x8::splat(8.0) * t187 * t751 - f64x8::splat(1.6891736332904388) * t758 * t763));
            let tvrho1 = t177 + t308 + t7 * (t647 + t767);
            acc_vrho_1 = tvrho1;
            let t776 = f64x8::splat(1.0) / t347;
            let t785 = ((t82).select(-t39 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t786 = t359 * t785;
            let t789 = t84 * t84;
            let t791 = f64x8::splat(1.0) / t375 / t96;
            let t792 = t789 * t791;
            let t793 = t76 * t792;
            let t794 = t389 * t101;
            let t795 = t105 * t785;
            let t796 = t794 * t795;
            let t799 = -f64x8::splat(3.0) / f64x8::splat(32.0) * t366 * t786 + t793 * t796 / f64x8::splat(256.0);
            let t800 = t66 * t799;
            let t803 = t799 * t120;
            let t806 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t67 * t33 * t39 + t800 * t120 / f64x8::splat(100.0) - t416 * t803 / f64x8::splat(100.0);
            let t809 = f64x8::splat(5.0) / f64x8::splat(972.0) * t34 * t39 + t50 * v_sigma0 * t55 * t62 / f64x8::splat(288.0) - f64x8::splat(9.0) / f64x8::splat(2560.0) * t345 * t51 * t776 * t62 + f64x8::splat(2.0) * t123 * t806;
            let t810 = t809 * t155;
            let t812 = -t799;
            let t813 = ((t132).select(f64x8::splat(0.0), t812));
            let t815 = t133 * t813;
            let t817 = t135 * t813;
            let t819 = t137 * t813;
            let t821 = t139 * t813;
            let t823 = t141 * t813;
            let t828 = ((t132).select(t812, f64x8::splat(0.0)));
            let t832 = ((t131).select(-f64x8::splat(0.667) * t813 - f64x8::splat(0.889111) * t815 - f64x8::splat(1.989259803147) * t817 + f64x8::splat(5.80518817796) * t819 - f64x8::splat(4.439990207985) * t821 + f64x8::splat(1.407173648874) * t823 - f64x8::splat(0.162300903254) * t143 * t813, -t442 * t444 * t828 * t152));
            let t835 = t328 * t810 - t130 * t832 + f64x8::splat(1.174) * t832;
            let t836 = t28 * t835;
            let t837 = t836 * t173;
            let t840 = f64x8::splat(1.0) / t163;
            let t843 = t162 * t840 * t165 * t172;
            let t847 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t837 + f64x8::splat(0.6334401124839145) * t465 * t843));
            let tvsigma0 = t7 * t847;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t854 = f64x8::splat(1.0) / t670;
            let t863 = ((t221).select(-t192 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t864 = t493 * t863;
            let t867 = t223 * t223;
            let t869 = f64x8::splat(1.0) / t500 / t232;
            let t870 = t867 * t869;
            let t871 = t215 * t870;
            let t872 = t514 * t101;
            let t873 = t239 * t863;
            let t874 = t872 * t873;
            let t877 = -f64x8::splat(3.0) / f64x8::splat(32.0) * t683 * t864 + t871 * t874 / f64x8::splat(256.0);
            let t878 = t66 * t877;
            let t881 = t877 * t254;
            let t884 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t67 * t33 * t192 + t878 * t254 / f64x8::splat(100.0) - t536 * t881 / f64x8::splat(100.0);
            let t887 = f64x8::splat(5.0) / f64x8::splat(972.0) * t34 * t192 + t50 * v_sigma2 * t200 * t206 / f64x8::splat(288.0) - f64x8::splat(9.0) / f64x8::splat(2560.0) * t345 * t196 * t854 * t206 + f64x8::splat(2.0) * t257 * t884;
            let t888 = t887 * t289;
            let t890 = -t877;
            let t891 = ((t266).select(f64x8::splat(0.0), t890));
            let t893 = t267 * t891;
            let t895 = t269 * t891;
            let t897 = t271 * t891;
            let t899 = t273 * t891;
            let t901 = t275 * t891;
            let t906 = ((t266).select(t890, f64x8::splat(0.0)));
            let t910 = ((t265).select(-f64x8::splat(0.667) * t891 - f64x8::splat(0.889111) * t893 - f64x8::splat(1.989259803147) * t895 + f64x8::splat(5.80518817796) * t897 - f64x8::splat(4.439990207985) * t899 + f64x8::splat(1.407173648874) * t901 - f64x8::splat(0.162300903254) * t277 * t891, -t442 * t562 * t906 * t286));
            let t913 = t490 * t888 - t264 * t910 + f64x8::splat(1.174) * t910;
            let t914 = t28 * t913;
            let t915 = t914 * t304;
            let t918 = f64x8::splat(1.0) / t294;
            let t921 = t162 * t918 * t296 * t303;
            let t925 = ((t178).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t187 * t915 + f64x8::splat(0.6334401124839145) * t758 * t921));
            let tvsigma2 = t7 * t925;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t926 = ((t82).select(t78, f64x8::splat(0.0)));
            let t927 = t359 * t926;
            let t930 = t105 * t926;
            let t931 = t794 * t930;
            let t934 = -f64x8::splat(3.0) / f64x8::splat(32.0) * t366 * t927 + t793 * t931 / f64x8::splat(256.0);
            let t935 = t66 * t934;
            let t937 = t934 * t120;
            let t940 = t935 * t120 / f64x8::splat(100.0) - t416 * t937 / f64x8::splat(100.0);
            let t941 = t123 * t940;
            let t945 = -t934;
            let t946 = ((t132).select(f64x8::splat(0.0), t945));
            let t948 = t133 * t946;
            let t950 = t135 * t946;
            let t952 = t137 * t946;
            let t954 = t139 * t946;
            let t956 = t141 * t946;
            let t961 = ((t132).select(t945, f64x8::splat(0.0)));
            let t965 = ((t131).select(-f64x8::splat(0.667) * t946 - f64x8::splat(0.889111) * t948 - f64x8::splat(1.989259803147) * t950 + f64x8::splat(5.80518817796) * t952 - f64x8::splat(4.439990207985) * t954 + f64x8::splat(1.407173648874) * t956 - f64x8::splat(0.162300903254) * t143 * t946, -t442 * t444 * t961 * t152));
            let t968 = f64x8::splat(2.0) * t328 * t941 * t155 - t130 * t965 + f64x8::splat(1.174) * t965;
            let t969 = t28 * t968;
            let t970 = t969 * t173;
            let t973 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t970));
            let tvtau0 = t7 * t973;
            acc_vtau_0 = tvtau0;
            let t974 = ((t221).select(t217, f64x8::splat(0.0)));
            let t975 = t493 * t974;
            let t978 = t239 * t974;
            let t979 = t872 * t978;
            let t982 = -f64x8::splat(3.0) / f64x8::splat(32.0) * t683 * t975 + t871 * t979 / f64x8::splat(256.0);
            let t983 = t66 * t982;
            let t985 = t982 * t254;
            let t988 = t983 * t254 / f64x8::splat(100.0) - t536 * t985 / f64x8::splat(100.0);
            let t989 = t257 * t988;
            let t993 = -t982;
            let t994 = ((t266).select(f64x8::splat(0.0), t993));
            let t996 = t267 * t994;
            let t998 = t269 * t994;
            let t1000 = t271 * t994;
            let t1002 = t273 * t994;
            let t1004 = t275 * t994;
            let t1009 = ((t266).select(t993, f64x8::splat(0.0)));
            let t1013 = ((t265).select(-f64x8::splat(0.667) * t994 - f64x8::splat(0.889111) * t996 - f64x8::splat(1.989259803147) * t998 + f64x8::splat(5.80518817796) * t1000 - f64x8::splat(4.439990207985) * t1002 + f64x8::splat(1.407173648874) * t1004 - f64x8::splat(0.162300903254) * t277 * t994, -t442 * t562 * t1009 * t286));
            let t1016 = f64x8::splat(2.0) * t490 * t989 * t289 - t264 * t1013 + f64x8::splat(1.174) * t1013;
            let t1017 = t28 * t1016;
            let t1018 = t1017 * t304;
            let t1021 = ((t178).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t187 * t1018));
            let tvtau1 = t7 * t1021;
            acc_vtau_1 = tvtau1;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(vlapl, ip, m, 2, 0, acc_vlapl_0);
        store_strided(vlapl, ip, m, 2, 1, acc_vlapl_1);
        store_strided(vtau, ip, m, 2, 0, acc_vtau_0);
        store_strided(vtau, ip, m, 2, 1, acc_vtau_1);
        ip += 8;
    }
}
