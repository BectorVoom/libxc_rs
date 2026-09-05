//! MGGA_X_R2SCAN vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_r2scan.c`
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
pub fn mgga_x_r2scan_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_c1: f64,
    param_c2: f64,
    param_d: f64,
    param_dp2: f64,
    param_eta: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c1 = f64x8::splat(param_c1);
    let param_c2 = f64x8::splat(param_c2);
    let param_d = f64x8::splat(param_d);
    let param_dp2 = f64x8::splat(param_dp2);
    let param_eta = f64x8::splat(param_eta);
    let param_k1 = f64x8::splat(param_k1);
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
            let t30 = f64x8::splat(20.0) / f64x8::splat(27.0) + f64x8::splat(5.0) / f64x8::splat(3.0) * param_eta;
            let t31 = f64x8::splat(M_CBRT6);
            let t32 = t31 * t31;
            let t33 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t34 = (simd::cbrt(t33));
            let t35 = t34 * t33;
            let t36 = f64x8::splat(1.0) / t35;
            let t37 = t32 * t36;
            let t38 = v_sigma0 * v_sigma0;
            let t39 = v_rho0 * v_rho0;
            let t40 = t39 * t39;
            let t41 = t40 * v_rho0;
            let t42 = (simd::cbrt(v_rho0));
            let t44 = f64x8::splat(1.0) / t42 / t41;
            let t45 = t38 * t44;
            let t46 = param_dp2 * param_dp2;
            let t47 = t46 * t46;
            let t48 = f64x8::splat(1.0) / t47;
            let t52 = (simd::exp(-t37 * t45 * t48 / f64x8::splat(576.0)));
            let t56 = (-f64x8::splat(0.162742215233874) * t30 * t52 + f64x8::splat(10.0) / f64x8::splat(81.0)) * t31;
            let t57 = t34 * t34;
            let t58 = f64x8::splat(1.0) / t57;
            let t59 = t58 * v_sigma0;
            let t60 = t42 * t42;
            let t61 = t60 * t39;
            let t62 = f64x8::splat(1.0) / t61;
            let t66 = param_k1 + t56 * t59 * t62 / f64x8::splat(24.0);
            let t70 = param_k1 * (f64x8::splat(1.0) - param_k1 / t66);
            let t71 = t60 * v_rho0;
            let t72 = f64x8::splat(1.0) / t71;
            let t74 = v_sigma0 * t62;
            let t76 = v_tau0 * t72 - t74 / f64x8::splat(8.0);
            let t78 = f64x8::splat(3.0) / f64x8::splat(10.0) * t32 * t57;
            let t79 = param_eta * v_sigma0;
            let t82 = t78 + t79 * t62 / f64x8::splat(8.0);
            let t83 = f64x8::splat(1.0) / t82;
            let t84 = t76 * t83;
            let t85 = (t84).simd_le(f64x8::splat(0.0));
            let t86 = (f64x8::splat(0.0)).simd_lt(t84);
            let t87 = ((t86).select(f64x8::splat(0.0), t84));
            let t88 = param_c1 * t87;
            let t89 = f64x8::splat(1.0) - t87;
            let t90 = f64x8::splat(1.0) / t89;
            let t92 = (simd::exp(-t88 * t90));
            let t93 = (t84).simd_le(f64x8::splat(2.5));
            let t94 = (f64x8::splat(2.5)).simd_lt(t84);
            let t95 = ((t94).select(f64x8::splat(2.5), t84));
            let t97 = t95 * t95;
            let t99 = t97 * t95;
            let t101 = t97 * t97;
            let t103 = t101 * t95;
            let t105 = t101 * t97;
            let t110 = ((t94).select(t84, f64x8::splat(2.5)));
            let t111 = f64x8::splat(1.0) - t110;
            let t114 = (simd::exp(param_c2 / t111));
            let t116 = ((t85).select(t92, (t93).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t95 - f64x8::splat(0.4445555) * t97 - f64x8::splat(0.663086601049) * t99 + f64x8::splat(1.45129704449) * t101 - f64x8::splat(0.887998041597) * t103 + f64x8::splat(0.234528941479) * t105 - f64x8::splat(0.023185843322) * t101 * t99, -param_d * t114)));
            let t117 = f64x8::splat(0.174) - t70;
            let t119 = t116 * t117 + t70 + f64x8::splat(1.0);
            let t120 = t28 * t119;
            let t121 = ((f64x8::splat(3.0)).sqrt());
            let t122 = f64x8::splat(1.0) / t34;
            let t123 = t32 * t122;
            let t124 = ((v_sigma0).sqrt());
            let t125 = t42 * v_rho0;
            let t126 = f64x8::splat(1.0) / t125;
            let t128 = t123 * t124 * t126;
            let t129 = ((t128).sqrt());
            let t133 = (simd::exp(-f64x8::splat(9.8958) * t121 / t129));
            let t134 = f64x8::splat(1.0) - t133;
            let t135 = t120 * t134;
            let t138 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t135));
            let t139 = (v_rho1).simd_le(dens_threshold);
            let t140 = -t17;
            let t142 = ((t15).select(t12, (t11).select(t16, t140 * t8)));
            let t143 = f64x8::splat(1.0) + t142;
            let t144 = (t143).simd_le(zeta_threshold);
            let t145 = (simd::cbrt(t143));
            let t147 = ((t144).select(t23, t145 * t143));
            let t148 = t6 * t147;
            let t149 = v_sigma2 * v_sigma2;
            let t150 = v_rho1 * v_rho1;
            let t151 = t150 * t150;
            let t152 = t151 * v_rho1;
            let t153 = (simd::cbrt(v_rho1));
            let t155 = f64x8::splat(1.0) / t153 / t152;
            let t156 = t149 * t155;
            let t160 = (simd::exp(-t37 * t156 * t48 / f64x8::splat(576.0)));
            let t164 = (-f64x8::splat(0.162742215233874) * t30 * t160 + f64x8::splat(10.0) / f64x8::splat(81.0)) * t31;
            let t165 = t58 * v_sigma2;
            let t166 = t153 * t153;
            let t167 = t166 * t150;
            let t168 = f64x8::splat(1.0) / t167;
            let t172 = param_k1 + t164 * t165 * t168 / f64x8::splat(24.0);
            let t176 = param_k1 * (f64x8::splat(1.0) - param_k1 / t172);
            let t177 = t166 * v_rho1;
            let t178 = f64x8::splat(1.0) / t177;
            let t180 = v_sigma2 * t168;
            let t182 = v_tau1 * t178 - t180 / f64x8::splat(8.0);
            let t183 = param_eta * v_sigma2;
            let t186 = t78 + t183 * t168 / f64x8::splat(8.0);
            let t187 = f64x8::splat(1.0) / t186;
            let t188 = t182 * t187;
            let t189 = (t188).simd_le(f64x8::splat(0.0));
            let t190 = (f64x8::splat(0.0)).simd_lt(t188);
            let t191 = ((t190).select(f64x8::splat(0.0), t188));
            let t192 = param_c1 * t191;
            let t193 = f64x8::splat(1.0) - t191;
            let t194 = f64x8::splat(1.0) / t193;
            let t196 = (simd::exp(-t192 * t194));
            let t197 = (t188).simd_le(f64x8::splat(2.5));
            let t198 = (f64x8::splat(2.5)).simd_lt(t188);
            let t199 = ((t198).select(f64x8::splat(2.5), t188));
            let t201 = t199 * t199;
            let t203 = t201 * t199;
            let t205 = t201 * t201;
            let t207 = t205 * t199;
            let t209 = t205 * t201;
            let t214 = ((t198).select(t188, f64x8::splat(2.5)));
            let t215 = f64x8::splat(1.0) - t214;
            let t218 = (simd::exp(param_c2 / t215));
            let t220 = ((t189).select(t196, (t197).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t199 - f64x8::splat(0.4445555) * t201 - f64x8::splat(0.663086601049) * t203 + f64x8::splat(1.45129704449) * t205 - f64x8::splat(0.887998041597) * t207 + f64x8::splat(0.234528941479) * t209 - f64x8::splat(0.023185843322) * t205 * t203, -param_d * t218)));
            let t221 = f64x8::splat(0.174) - t176;
            let t223 = t220 * t221 + t176 + f64x8::splat(1.0);
            let t224 = t28 * t223;
            let t225 = ((v_sigma2).sqrt());
            let t226 = t153 * v_rho1;
            let t227 = f64x8::splat(1.0) / t226;
            let t229 = t123 * t225 * t227;
            let t230 = ((t229).sqrt());
            let t234 = (simd::exp(-f64x8::splat(9.8958) * t121 / t230));
            let t235 = f64x8::splat(1.0) - t234;
            let t236 = t224 * t235;
            let t239 = ((t139).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t148 * t236));
            let tzk0 = t138 + t239;
            acc_zk = tzk0;
            let t240 = t7 * t7;
            let t241 = f64x8::splat(1.0) / t240;
            let t242 = t17 * t241;
            let t244 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t242)));
            let t247 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t244));
            let t248 = t6 * t247;
            let t251 = t28 * t28;
            let t252 = f64x8::splat(1.0) / t251;
            let t253 = t252 * t119;
            let t254 = t253 * t134;
            let t256 = t27 * t254 / f64x8::splat(8.0);
            let t257 = param_k1 * param_k1;
            let t258 = t66 * t66;
            let t259 = f64x8::splat(1.0) / t258;
            let t260 = t257 * t259;
            let t261 = t38 * v_sigma0;
            let t262 = t30 * t261;
            let t263 = t40 * t40;
            let t264 = t263 * v_rho0;
            let t265 = f64x8::splat(1.0) / t264;
            let t267 = t265 * t48 * t52;
            let t270 = t39 * v_rho0;
            let t272 = f64x8::splat(1.0) / t60 / t270;
            let t276 = -f64x8::splat(3.867381235367984e-06) * t262 * t267 - t56 * t59 * t272 / f64x8::splat(9.0);
            let t282 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau0 * t62 + v_sigma0 * t272 / f64x8::splat(3.0);
            let t284 = t82 * t82;
            let t285 = f64x8::splat(1.0) / t284;
            let t286 = t76 * t285;
            let t287 = t79 * t272;
            let t290 = t282 * t83 + t286 * t287 / f64x8::splat(3.0);
            let t291 = ((t86).select(f64x8::splat(0.0), t290));
            let t294 = t89 * t89;
            let t295 = f64x8::splat(1.0) / t294;
            let t296 = t295 * t291;
            let t298 = -t291 * t90 * param_c1 - t296 * t88;
            let t299 = t298 * t92;
            let t300 = ((t94).select(f64x8::splat(0.0), t290));
            let t302 = t95 * t300;
            let t304 = t97 * t300;
            let t306 = t99 * t300;
            let t308 = t101 * t300;
            let t310 = t103 * t300;
            let t315 = param_d * param_c2;
            let t316 = t111 * t111;
            let t317 = f64x8::splat(1.0) / t316;
            let t318 = ((t94).select(t290, f64x8::splat(0.0)));
            let t322 = ((t85).select(t299, (t93).select(-f64x8::splat(0.667) * t300 - f64x8::splat(0.889111) * t302 - f64x8::splat(1.989259803147) * t304 + f64x8::splat(5.80518817796) * t306 - f64x8::splat(4.439990207985) * t308 + f64x8::splat(1.407173648874) * t310 - f64x8::splat(0.162300903254) * t105 * t300, -t315 * t317 * t318 * t114)));
            let t324 = t116 * t257;
            let t325 = t259 * t276;
            let t327 = t117 * t322 + t260 * t276 - t324 * t325;
            let t328 = t28 * t327;
            let t329 = t328 * t134;
            let t332 = (simd::pow(f64x8::splat(3.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t333 = t332 * t332;
            let t334 = t333 * t333;
            let t335 = t334 * t332;
            let t336 = t335 * t26;
            let t338 = f64x8::splat(1.0) / t129 / t128;
            let t339 = t120 * t338;
            let t340 = t336 * t339;
            let t342 = f64x8::splat(1.0) / t42 / t39;
            let t344 = t124 * t342 * t133;
            let t345 = t123 * t344;
            let t349 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t248 * t135 - t256 - f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t329 - f64x8::splat(1.6891736332904388) * t340 * t345));
            let t350 = t140 * t241;
            let t352 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t350)));
            let t355 = ((t144).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t145 * t352));
            let t356 = t6 * t355;
            let t359 = t252 * t223;
            let t360 = t359 * t235;
            let t362 = t148 * t360 / f64x8::splat(8.0);
            let t364 = ((t139).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t356 * t236 - t362));
            let tvrho0 = t138 + t239 + t7 * (t349 + t364);
            acc_vrho_0 = tvrho0;
            let t368 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t242)));
            let t371 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t368));
            let t372 = t6 * t371;
            let t376 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t372 * t135 - t256));
            let t378 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t350)));
            let t381 = ((t144).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t145 * t378));
            let t382 = t6 * t381;
            let t385 = t172 * t172;
            let t386 = f64x8::splat(1.0) / t385;
            let t387 = t257 * t386;
            let t388 = t149 * v_sigma2;
            let t389 = t30 * t388;
            let t390 = t151 * t151;
            let t391 = t390 * v_rho1;
            let t392 = f64x8::splat(1.0) / t391;
            let t394 = t392 * t48 * t160;
            let t397 = t150 * v_rho1;
            let t399 = f64x8::splat(1.0) / t166 / t397;
            let t403 = -f64x8::splat(3.867381235367984e-06) * t389 * t394 - t164 * t165 * t399 / f64x8::splat(9.0);
            let t409 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau1 * t168 + v_sigma2 * t399 / f64x8::splat(3.0);
            let t411 = t186 * t186;
            let t412 = f64x8::splat(1.0) / t411;
            let t413 = t182 * t412;
            let t414 = t183 * t399;
            let t417 = t409 * t187 + t413 * t414 / f64x8::splat(3.0);
            let t418 = ((t190).select(f64x8::splat(0.0), t417));
            let t421 = t193 * t193;
            let t422 = f64x8::splat(1.0) / t421;
            let t423 = t422 * t418;
            let t425 = -t194 * t418 * param_c1 - t192 * t423;
            let t426 = t425 * t196;
            let t427 = ((t198).select(f64x8::splat(0.0), t417));
            let t429 = t199 * t427;
            let t431 = t201 * t427;
            let t433 = t203 * t427;
            let t435 = t205 * t427;
            let t437 = t207 * t427;
            let t442 = t215 * t215;
            let t443 = f64x8::splat(1.0) / t442;
            let t444 = ((t198).select(t417, f64x8::splat(0.0)));
            let t448 = ((t189).select(t426, (t197).select(-f64x8::splat(0.667) * t427 - f64x8::splat(0.889111) * t429 - f64x8::splat(1.989259803147) * t431 + f64x8::splat(5.80518817796) * t433 - f64x8::splat(4.439990207985) * t435 + f64x8::splat(1.407173648874) * t437 - f64x8::splat(0.162300903254) * t209 * t427, -t315 * t443 * t444 * t218)));
            let t450 = t220 * t257;
            let t451 = t386 * t403;
            let t453 = t221 * t448 + t387 * t403 - t450 * t451;
            let t454 = t28 * t453;
            let t455 = t454 * t235;
            let t458 = t335 * t147;
            let t460 = f64x8::splat(1.0) / t230 / t229;
            let t461 = t224 * t460;
            let t462 = t458 * t461;
            let t464 = f64x8::splat(1.0) / t153 / t150;
            let t466 = t225 * t464 * t234;
            let t467 = t123 * t466;
            let t471 = ((t139).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t382 * t236 - t362 - f64x8::splat(3.0) / f64x8::splat(8.0) * t148 * t455 - f64x8::splat(1.6891736332904388) * t462 * t467));
            let tvrho1 = t138 + t239 + t7 * (t376 + t471);
            acc_vrho_1 = tvrho1;
            let t474 = t30 * t38;
            let t475 = f64x8::splat(1.0) / t263;
            let t477 = t475 * t48 * t52;
            let t483 = f64x8::splat(1.450267963262994e-06) * t474 * t477 + t56 * t58 * t62 / f64x8::splat(24.0);
            let t485 = t62 * t83;
            let t486 = param_eta * t62;
            let t489 = -t286 * t486 / f64x8::splat(8.0) - t485 / f64x8::splat(8.0);
            let t490 = ((t86).select(f64x8::splat(0.0), t489));
            let t491 = param_c1 * t490;
            let t493 = t295 * t490;
            let t495 = -t491 * t90 - t493 * t88;
            let t496 = t495 * t92;
            let t497 = ((t94).select(f64x8::splat(0.0), t489));
            let t499 = t95 * t497;
            let t501 = t97 * t497;
            let t503 = t99 * t497;
            let t505 = t101 * t497;
            let t507 = t103 * t497;
            let t512 = ((t94).select(t489, f64x8::splat(0.0)));
            let t516 = ((t85).select(t496, (t93).select(-f64x8::splat(0.667) * t497 - f64x8::splat(0.889111) * t499 - f64x8::splat(1.989259803147) * t501 + f64x8::splat(5.80518817796) * t503 - f64x8::splat(4.439990207985) * t505 + f64x8::splat(1.407173648874) * t507 - f64x8::splat(0.162300903254) * t105 * t497, -t315 * t317 * t512 * t114)));
            let t518 = t259 * t483;
            let t520 = t117 * t516 + t260 * t483 - t324 * t518;
            let t521 = t28 * t520;
            let t522 = t521 * t134;
            let t525 = f64x8::splat(1.0) / t124;
            let t527 = t525 * t126 * t133;
            let t528 = t123 * t527;
            let t532 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t522 + f64x8::splat(0.6334401124839145) * t340 * t528));
            let tvsigma0 = t7 * t532;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t533 = t30 * t149;
            let t534 = f64x8::splat(1.0) / t390;
            let t536 = t534 * t48 * t160;
            let t542 = f64x8::splat(1.450267963262994e-06) * t533 * t536 + t164 * t58 * t168 / f64x8::splat(24.0);
            let t544 = t168 * t187;
            let t545 = param_eta * t168;
            let t548 = -t413 * t545 / f64x8::splat(8.0) - t544 / f64x8::splat(8.0);
            let t549 = ((t190).select(f64x8::splat(0.0), t548));
            let t550 = param_c1 * t549;
            let t552 = t422 * t549;
            let t554 = -t192 * t552 - t194 * t550;
            let t555 = t554 * t196;
            let t556 = ((t198).select(f64x8::splat(0.0), t548));
            let t558 = t199 * t556;
            let t560 = t201 * t556;
            let t562 = t203 * t556;
            let t564 = t205 * t556;
            let t566 = t207 * t556;
            let t571 = ((t198).select(t548, f64x8::splat(0.0)));
            let t575 = ((t189).select(t555, (t197).select(-f64x8::splat(0.667) * t556 - f64x8::splat(0.889111) * t558 - f64x8::splat(1.989259803147) * t560 + f64x8::splat(5.80518817796) * t562 - f64x8::splat(4.439990207985) * t564 + f64x8::splat(1.407173648874) * t566 - f64x8::splat(0.162300903254) * t209 * t556, -t315 * t443 * t571 * t218)));
            let t577 = t386 * t542;
            let t579 = t221 * t575 + t387 * t542 - t450 * t577;
            let t580 = t28 * t579;
            let t581 = t580 * t235;
            let t584 = f64x8::splat(1.0) / t225;
            let t586 = t584 * t227 * t234;
            let t587 = t123 * t586;
            let t591 = ((t139).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t148 * t581 + f64x8::splat(0.6334401124839145) * t462 * t587));
            let tvsigma2 = t7 * t591;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t592 = t72 * t83;
            let t593 = ((t86).select(f64x8::splat(0.0), t592));
            let t594 = param_c1 * t593;
            let t596 = t295 * t593;
            let t598 = -t594 * t90 - t596 * t88;
            let t599 = t598 * t92;
            let t600 = ((t94).select(f64x8::splat(0.0), t592));
            let t602 = t95 * t600;
            let t604 = t97 * t600;
            let t606 = t99 * t600;
            let t608 = t101 * t600;
            let t610 = t103 * t600;
            let t615 = ((t94).select(t592, f64x8::splat(0.0)));
            let t619 = ((t85).select(t599, (t93).select(-f64x8::splat(0.667) * t600 - f64x8::splat(0.889111) * t602 - f64x8::splat(1.989259803147) * t604 + f64x8::splat(5.80518817796) * t606 - f64x8::splat(4.439990207985) * t608 + f64x8::splat(1.407173648874) * t610 - f64x8::splat(0.162300903254) * t105 * t600, -t315 * t317 * t615 * t114)));
            let t620 = t28 * t619;
            let t621 = t117 * t134;
            let t622 = t620 * t621;
            let t625 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t622));
            let tvtau0 = t7 * t625;
            acc_vtau_0 = tvtau0;
            let t626 = t178 * t187;
            let t627 = ((t190).select(f64x8::splat(0.0), t626));
            let t628 = param_c1 * t627;
            let t630 = t422 * t627;
            let t632 = -t192 * t630 - t194 * t628;
            let t633 = t632 * t196;
            let t634 = ((t198).select(f64x8::splat(0.0), t626));
            let t636 = t199 * t634;
            let t638 = t201 * t634;
            let t640 = t203 * t634;
            let t642 = t205 * t634;
            let t644 = t207 * t634;
            let t649 = ((t198).select(t626, f64x8::splat(0.0)));
            let t653 = ((t189).select(t633, (t197).select(-f64x8::splat(0.667) * t634 - f64x8::splat(0.889111) * t636 - f64x8::splat(1.989259803147) * t638 + f64x8::splat(5.80518817796) * t640 - f64x8::splat(4.439990207985) * t642 + f64x8::splat(1.407173648874) * t644 - f64x8::splat(0.162300903254) * t209 * t634, -t315 * t443 * t649 * t218)));
            let t654 = t28 * t653;
            let t655 = t221 * t235;
            let t656 = t654 * t655;
            let t659 = ((t139).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t148 * t656));
            let tvtau1 = t7 * t659;
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
