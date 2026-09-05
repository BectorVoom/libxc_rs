//! MGGA_C_R2SCAN vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_r2scan.c`
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
pub fn mgga_c_r2scan_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_eta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_eta = f64x8::splat(param_eta);
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
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t4 = (simd::cbrt(t3));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t8 = v_rho0 + v_rho1;
            let t9 = (simd::cbrt(t8));
            let t11 = t7 / t9;
            let t12 = t5 * t11;
            let t14 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t12;
            let t15 = ((t12).sqrt());
            let t17 = f64x8::splat(0.8969) * t12;
            let t18 = ((t12) * (t12).sqrt());
            let t19 = f64x8::splat(0.204775) * t18;
            let t20 = t2 * t2;
            let t21 = t4 * t4;
            let t22 = t20 * t21;
            let t23 = t9 * t9;
            let t26 = t22 * t6 / t23;
            let t27 = f64x8::splat(0.123235) * t26;
            let t28 = f64x8::splat(3.79785) * t15 + t17 + t19 + t27;
            let t31 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t28;
            let t32 = (simd::ln(t31));
            let t34 = f64x8::splat(0.0621814) * t14 * t32;
            let t35 = v_rho0 - v_rho1;
            let t36 = t35 * t35;
            let t37 = t36 * t36;
            let t38 = t8 * t8;
            let t39 = t38 * t38;
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t37 * t40;
            let t42 = f64x8::splat(1.0) / t8;
            let t43 = t35 * t42;
            let t44 = f64x8::splat(1.0) + t43;
            let t45 = (t44).simd_le(zeta_threshold);
            let t46 = (simd::cbrt(zeta_threshold));
            let t47 = t46 * zeta_threshold;
            let t48 = (simd::cbrt(t44));
            let t49 = t48 * t44;
            let t50 = ((t45).select(t47, t49));
            let t51 = f64x8::splat(1.0) - t43;
            let t52 = (t51).simd_le(zeta_threshold);
            let t53 = (simd::cbrt(t51));
            let t54 = t53 * t51;
            let t55 = ((t52).select(t47, t54));
            let t56 = t50 + t55 - f64x8::splat(2.0);
            let t57 = f64x8::splat(M_CBRT2);
            let t58 = t57 - f64x8::splat(1.0);
            let t60 = f64x8::splat(1.0) / t58 / f64x8::splat(2.0);
            let t61 = t56 * t60;
            let t63 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t12;
            let t65 = f64x8::splat(1.549425) * t12;
            let t66 = f64x8::splat(0.420775) * t18;
            let t67 = f64x8::splat(0.1562925) * t26;
            let t68 = f64x8::splat(7.05945) * t15 + t65 + t66 + t67;
            let t71 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t68;
            let t72 = (simd::ln(t71));
            let t76 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t12;
            let t78 = f64x8::splat(0.905775) * t12;
            let t79 = f64x8::splat(0.1100325) * t18;
            let t80 = f64x8::splat(0.1241775) * t26;
            let t81 = f64x8::splat(5.1785) * t15 + t78 + t79 + t80;
            let t84 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t81;
            let t85 = (simd::ln(t84));
            let t86 = t76 * t85;
            let t88 = -f64x8::splat(0.0310907) * t63 * t72 + t34 - f64x8::splat(0.0197516734986138) * t86;
            let t89 = t61 * t88;
            let t90 = t41 * t89;
            let t92 = f64x8::splat(0.0197516734986138) * t61 * t86;
            let t93 = (simd::ln(f64x8::splat(2.0)));
            let t94 = f64x8::splat(1.0) - t93;
            let t95 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t97 = t94 / t95;
            let t98 = t46 * t46;
            let t99 = t48 * t48;
            let t100 = ((t45).select(t98, t99));
            let t101 = t53 * t53;
            let t102 = ((t52).select(t98, t101));
            let t104 = t100 / f64x8::splat(2.0) + t102 / f64x8::splat(2.0);
            let t105 = t104 * t104;
            let t106 = t105 * t104;
            let t108 = f64x8::splat(1.0) / t94;
            let t109 = (-t34 + t90 + t92) * t108;
            let t110 = f64x8::splat(1.0) / t106;
            let t111 = t95 * t110;
            let t113 = (simd::exp(-t109 * t111));
            let t114 = t113 - f64x8::splat(1.0);
            let t116 = f64x8::splat(1.0) + f64x8::splat(0.025) * t12;
            let t118 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t12;
            let t119 = f64x8::splat(1.0) / t118;
            let t120 = t116 * t119;
            let t122 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t124 = f64x8::splat(1.0) / t9 / t38;
            let t125 = t122 * t124;
            let t128 = f64x8::splat(1.0) / t105;
            let t129 = t128 * t20;
            let t130 = f64x8::splat(1.0) / t4;
            let t132 = t6 * t108;
            let t133 = f64x8::splat(1.0) / t114;
            let t134 = t132 * t133;
            let t135 = t129 * t130 * t134;
            let t138 = t98 * zeta_threshold;
            let t139 = t99 * t44;
            let t140 = ((t45).select(t138, t139));
            let t141 = t101 * t51;
            let t142 = ((t52).select(t138, t141));
            let t144 = t140 / f64x8::splat(2.0) + t142 / f64x8::splat(2.0);
            let t146 = t108 / t144;
            let t147 = t110 * t133;
            let t148 = ((f64x8::splat(4.0)).sqrt());
            let t149 = t148 * t15;
            let t151 = f64x8::splat(0.03138525) * t12;
            let t152 = f64x8::splat(1.0) + f64x8::splat(0.022225) * t149 + t151;
            let t153 = t152 * t152;
            let t154 = f64x8::splat(1.0) / t153;
            let t158 = f64x8::splat(1.0) - f64x8::splat(2.363) * t58 * t56 * t60;
            let t159 = t154 * t158;
            let t160 = t37 * t37;
            let t161 = t160 * t37;
            let t162 = t39 * t39;
            let t163 = t162 * t39;
            let t164 = f64x8::splat(1.0) / t163;
            let t166 = -t161 * t164 + f64x8::splat(1.0);
            let t167 = f64x8::splat(1.0) / t15;
            let t168 = t148 * t167;
            let t170 = f64x8::splat(0.04445) * t168 + f64x8::splat(0.125541);
            let t171 = t166 * t170;
            let t175 = f64x8::splat(1.898925) * t149 + t17 + t19 + t27;
            let t178 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t175;
            let t179 = (simd::ln(t178));
            let t180 = f64x8::splat(0.01328816518) * t179;
            let t181 = t175 * t175;
            let t182 = f64x8::splat(1.0) / t181;
            let t183 = t14 * t182;
            let t185 = ((t12).sqrt());
            let t188 = f64x8::splat(3.79785) * t168 + f64x8::splat(3.5876) + f64x8::splat(1.22865) * t185 + f64x8::splat(0.24647) * t12;
            let t189 = f64x8::splat(1.0) / t178;
            let t190 = t188 * t189;
            let t192 = f64x8::splat(1.0) * t183 * t190;
            let t194 = f64x8::splat(3.529725) * t149 + t65 + t66 + t67;
            let t197 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t194;
            let t198 = (simd::ln(t197));
            let t200 = t194 * t194;
            let t201 = f64x8::splat(1.0) / t200;
            let t202 = t63 * t201;
            let t206 = f64x8::splat(7.05945) * t168 + f64x8::splat(6.1977) + f64x8::splat(2.52465) * t185 + f64x8::splat(0.312585) * t12;
            let t207 = f64x8::splat(1.0) / t197;
            let t208 = t206 * t207;
            let t212 = f64x8::splat(2.58925) * t149 + t78 + t79 + t80;
            let t215 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t212;
            let t216 = (simd::ln(t215));
            let t218 = t212 * t212;
            let t219 = f64x8::splat(1.0) / t218;
            let t220 = t76 * t219;
            let t224 = f64x8::splat(5.1785) * t168 + f64x8::splat(3.6231) + f64x8::splat(0.660195) * t185 + f64x8::splat(0.248355) * t12;
            let t225 = f64x8::splat(1.0) / t215;
            let t226 = t224 * t225;
            let t229 = -f64x8::splat(0.006388517036) * t198 + f64x8::splat(1.0) * t202 * t208 + t180 - t192 - f64x8::splat(0.0021973736767207856) * t216 + f64x8::splat(0.5848223622634646) * t220 * t226;
            let t230 = t61 * t229;
            let t234 = t61 * t76;
            let t236 = t219 * t224 * t225;
            let t239 = f64x8::splat(0.0285764) * t159 * t171 + t180 - t192 - t41 * t230 - f64x8::splat(0.0021973736767207856) * t61 * t216 + f64x8::splat(0.5848223622634646) * t234 * t236;
            let t244 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t15 + t151;
            let t245 = f64x8::splat(1.0) / t244;
            let t246 = t245 * t158;
            let t252 = f64x8::splat(5.0) * t5 * t11 * t239 - f64x8::splat(45.0) * param_eta * (-f64x8::splat(0.0285764) * t246 * t166 + t34 - t90 - t92);
            let t253 = t147 * t252;
            let t254 = t146 * t253;
            let t255 = f64x8::splat(M_CBRT6);
            let t256 = (simd::cbrt(t95));
            let t257 = t256 * t256;
            let t258 = f64x8::splat(1.0) / t257;
            let t259 = t255 * t258;
            let t260 = t57 * t57;
            let t261 = t259 * t260;
            let t263 = f64x8::splat(1.0) / t23 / t38;
            let t264 = t122 * t263;
            let t265 = t255 * t255;
            let t267 = f64x8::splat(1.0) / t256 / t95;
            let t268 = t265 * t267;
            let t269 = t122 * t122;
            let t270 = t57 * t269;
            let t271 = t39 * t8;
            let t273 = f64x8::splat(1.0) / t9 / t271;
            let t277 = (simd::exp(-f64x8::splat(0.2044460407889637) * t268 * t270 * t273));
            let t279 = t261 * t264 * t277;
            let t282 = f64x8::splat(1.0) + f64x8::splat(0.027439371595564633) * t120 * t125 * t57 * t135 + f64x8::splat(0.043341108700271344) * t254 * t279;
            let t283 = ((t282).sqrt().sqrt());
            let t285 = f64x8::splat(1.0) - f64x8::splat(1.0) / t283;
            let t287 = t114 * t285 + f64x8::splat(1.0);
            let t288 = (simd::ln(t287));
            let t290 = t97 * t106 * t288;
            let t291 = (simd::cbrt(v_rho0));
            let t292 = t291 * t291;
            let t294 = f64x8::splat(1.0) / t292 / v_rho0;
            let t295 = v_tau0 * t294;
            let t296 = t44 / f64x8::splat(2.0);
            let t297 = (simd::cbrt(t296));
            let t298 = t297 * t297;
            let t299 = t298 * t296;
            let t301 = (simd::cbrt(v_rho1));
            let t302 = t301 * t301;
            let t304 = f64x8::splat(1.0) / t302 / v_rho1;
            let t305 = v_tau1 * t304;
            let t306 = t51 / f64x8::splat(2.0);
            let t307 = (simd::cbrt(t306));
            let t308 = t307 * t307;
            let t309 = t308 * t306;
            let t312 = t295 * t299 + t305 * t309 - t264 / f64x8::splat(8.0);
            let t313 = t265 * t257;
            let t317 = param_eta * t122;
            let t320 = f64x8::splat(3.0) / f64x8::splat(10.0) * t313 * (t299 + t309) + t317 * t263 / f64x8::splat(8.0);
            let t321 = f64x8::splat(1.0) / t320;
            let t322 = t312 * t321;
            let t323 = (t322).simd_le(f64x8::splat(0.0));
            let t324 = (f64x8::splat(0.0)).simd_lt(t322);
            let t325 = ((t324).select(f64x8::splat(0.0), t322));
            let t326 = f64x8::splat(1.0) - t325;
            let t327 = f64x8::splat(1.0) / t326;
            let t330 = (simd::exp(-f64x8::splat(0.64) * t325 * t327));
            let t331 = (t322).simd_le(f64x8::splat(2.5));
            let t332 = (f64x8::splat(2.5)).simd_lt(t322);
            let t333 = ((t332).select(f64x8::splat(2.5), t322));
            let t335 = t333 * t333;
            let t337 = t335 * t333;
            let t339 = t335 * t335;
            let t341 = t339 * t333;
            let t343 = t339 * t335;
            let t348 = ((t332).select(t322, f64x8::splat(2.5)));
            let t349 = f64x8::splat(1.0) - t348;
            let t352 = (simd::exp(f64x8::splat(1.5) / t349));
            let t354 = ((t323).select(t330, (t331).select(f64x8::splat(1.0) - f64x8::splat(0.64) * t333 - f64x8::splat(0.4352) * t335 - f64x8::splat(1.535685604549) * t337 + f64x8::splat(3.061560252175) * t339 - f64x8::splat(1.915710236206) * t341 + f64x8::splat(0.516884468372) * t343 - f64x8::splat(0.051848879792) * t339 * t337, -f64x8::splat(0.7) * t352)));
            let t357 = (simd::exp(f64x8::splat(1.0) * t245));
            let t358 = t357 - f64x8::splat(1.0);
            let t359 = t260 * t122;
            let t360 = t359 * t263;
            let t363 = f64x8::splat(1.0) + f64x8::splat(0.02133764210437636) * t259 * t360;
            let t364 = ((t363).sqrt().sqrt());
            let t366 = f64x8::splat(1.0) - f64x8::splat(1.0) / t364;
            let t368 = t358 * t366 + f64x8::splat(1.0);
            let t369 = (simd::ln(t368));
            let t371 = -f64x8::splat(0.0285764) * t245 + f64x8::splat(0.0285764) * t369;
            let t372 = t371 * t158;
            let t374 = t372 * t166 - t290 + t34 - t90 - t92;
            let t375 = t354 * t374;
            let tzk0 = -t34 + t90 + t92 + t290 + t375;
            acc_zk = tzk0;
            let t377 = f64x8::splat(1.0) / t9 / t8;
            let t378 = t7 * t377;
            let t380 = t5 * t378 * t32;
            let t381 = f64x8::splat(0.0011073470983333333) * t380;
            let t382 = t28 * t28;
            let t383 = f64x8::splat(1.0) / t382;
            let t384 = t14 * t383;
            let t385 = t167 * t2;
            let t386 = t4 * t7;
            let t387 = t386 * t377;
            let t388 = t385 * t387;
            let t390 = t5 * t378;
            let t391 = f64x8::splat(0.29896666666666666) * t390;
            let t392 = t185 * t2;
            let t393 = t392 * t387;
            let t394 = f64x8::splat(0.1023875) * t393;
            let t398 = t22 * t6 / t23 / t8;
            let t399 = f64x8::splat(0.08215666666666667) * t398;
            let t400 = -f64x8::splat(0.632975) * t388 - t391 - t394 - t399;
            let t401 = f64x8::splat(1.0) / t31;
            let t402 = t400 * t401;
            let t403 = t384 * t402;
            let t404 = f64x8::splat(1.0) * t403;
            let t405 = t36 * t35;
            let t406 = t405 * t40;
            let t407 = t406 * t89;
            let t408 = f64x8::splat(4.0) * t407;
            let t409 = f64x8::splat(1.0) / t271;
            let t410 = t37 * t409;
            let t411 = t410 * t89;
            let t412 = f64x8::splat(4.0) * t411;
            let t413 = f64x8::splat(1.0) / t38;
            let t414 = t35 * t413;
            let t415 = t42 - t414;
            let t418 = ((t45).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t48 * t415));
            let t419 = -t415;
            let t422 = ((t52).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t53 * t419));
            let t423 = t418 + t422;
            let t424 = t423 * t60;
            let t425 = t424 * t88;
            let t426 = t41 * t425;
            let t430 = t68 * t68;
            let t431 = f64x8::splat(1.0) / t430;
            let t432 = t63 * t431;
            let t434 = f64x8::splat(0.516475) * t390;
            let t435 = f64x8::splat(0.2103875) * t393;
            let t436 = f64x8::splat(0.104195) * t398;
            let t437 = -f64x8::splat(1.176575) * t388 - t434 - t435 - t436;
            let t438 = f64x8::splat(1.0) / t71;
            let t439 = t437 * t438;
            let t445 = t81 * t81;
            let t446 = f64x8::splat(1.0) / t445;
            let t447 = t76 * t446;
            let t449 = f64x8::splat(0.301925) * t390;
            let t450 = f64x8::splat(0.05501625) * t393;
            let t451 = f64x8::splat(0.082785) * t398;
            let t452 = -f64x8::splat(0.8630833333333333) * t388 - t449 - t450 - t451;
            let t453 = f64x8::splat(1.0) / t84;
            let t454 = t452 * t453;
            let t457 = f64x8::splat(0.0005323764196666666) * t5 * t378 * t72 + f64x8::splat(1.0) * t432 * t439 - t381 - t404 + f64x8::splat(0.00018311447306006544) * t5 * t378 * t85 + f64x8::splat(0.5848223622634646) * t447 * t454;
            let t458 = t61 * t457;
            let t459 = t41 * t458;
            let t460 = t424 * t86;
            let t461 = f64x8::splat(0.0197516734986138) * t460;
            let t462 = t61 * t2;
            let t464 = t386 * t377 * t85;
            let t465 = t462 * t464;
            let t466 = f64x8::splat(0.00018311447306006544) * t465;
            let t468 = t446 * t452 * t453;
            let t469 = t234 * t468;
            let t470 = f64x8::splat(0.5848223622634646) * t469;
            let t471 = t105 * t288;
            let t472 = f64x8::splat(1.0) / t48;
            let t473 = t472 * t415;
            let t475 = ((t45).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t473));
            let t476 = f64x8::splat(1.0) / t53;
            let t477 = t476 * t419;
            let t479 = ((t52).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t477));
            let t481 = t475 / f64x8::splat(2.0) + t479 / f64x8::splat(2.0);
            let t483 = t97 * t471 * t481;
            let t484 = f64x8::splat(3.0) * t483;
            let t486 = (t381 + t404 + t408 - t412 + t426 + t459 + t461 - t466 - t470) * t108;
            let t488 = t105 * t105;
            let t489 = f64x8::splat(1.0) / t488;
            let t490 = t95 * t489;
            let t491 = t490 * t481;
            let t494 = f64x8::splat(3.0) * t109 * t491 - t486 * t111;
            let t495 = t494 * t113;
            let t496 = t495 * t285;
            let t498 = f64x8::splat(1.0) / t283 / t282;
            let t499 = t114 * t498;
            let t500 = t38 * t8;
            let t502 = f64x8::splat(1.0) / t23 / t500;
            let t503 = t502 * t119;
            let t505 = t57 * t128;
            let t506 = t108 * t133;
            let t507 = t505 * t506;
            let t509 = f64x8::splat(0.002743937159556463) * t503 * t122 * t507;
            let t510 = t118 * t118;
            let t511 = f64x8::splat(1.0) / t510;
            let t512 = t116 * t511;
            let t513 = t122 * t502;
            let t514 = t512 * t513;
            let t516 = f64x8::splat(0.004878720269691391) * t514 * t507;
            let t518 = f64x8::splat(1.0) / t9 / t500;
            let t523 = f64x8::splat(0.0640252003896508) * t120 * t122 * t518 * t57 * t135;
            let t524 = t120 * t122;
            let t525 = t124 * t57;
            let t526 = t525 * t110;
            let t527 = t524 * t526;
            let t528 = t20 * t130;
            let t529 = t528 * t6;
            let t530 = t506 * t481;
            let t531 = t529 * t530;
            let t534 = t525 * t128;
            let t535 = t524 * t534;
            let t536 = t114 * t114;
            let t537 = f64x8::splat(1.0) / t536;
            let t538 = t108 * t537;
            let t539 = t538 * t495;
            let t540 = t529 * t539;
            let t543 = t144 * t144;
            let t545 = t108 / t543;
            let t546 = t545 * t110;
            let t547 = t133 * t252;
            let t548 = t547 * t255;
            let t549 = t546 * t548;
            let t550 = t258 * t260;
            let t551 = t550 * t122;
            let t552 = t263 * t277;
            let t555 = ((t45).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t99 * t415));
            let t558 = ((t52).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t101 * t419));
            let t560 = t555 / f64x8::splat(2.0) + t558 / f64x8::splat(2.0);
            let t561 = t552 * t560;
            let t562 = t551 * t561;
            let t565 = t146 * t489;
            let t566 = t565 * t548;
            let t567 = t552 * t481;
            let t568 = t551 * t567;
            let t571 = t146 * t110;
            let t572 = t537 * t252;
            let t573 = t572 * t255;
            let t574 = t571 * t573;
            let t576 = t551 * t552 * t495;
            let t581 = f64x8::splat(5.0) / f64x8::splat(3.0) * t5 * t378 * t239;
            let t583 = f64x8::splat(1.0) / t153 / t152;
            let t584 = t583 * t158;
            let t585 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t586 = t585 * t167;
            let t587 = t5 * t377;
            let t588 = t586 * t587;
            let t590 = f64x8::splat(0.01046175) * t390;
            let t591 = -f64x8::splat(0.014816666666666667) * t588 - t590;
            let t594 = f64x8::splat(0.0571528) * t584 * t171 * t591;
            let t595 = t154 * t58;
            let t596 = t595 * t423;
            let t597 = t60 * t166;
            let t598 = t597 * t170;
            let t601 = t160 * t405;
            let t602 = t601 * t164;
            let t603 = t162 * t271;
            let t604 = f64x8::splat(1.0) / t603;
            let t605 = t161 * t604;
            let t607 = -f64x8::splat(12.0) * t602 + f64x8::splat(12.0) * t605;
            let t608 = t607 * t170;
            let t611 = t166 * t585;
            let t612 = t159 * t611;
            let t614 = f64x8::splat(1.0) / t15 / t12;
            let t615 = t614 * t2;
            let t616 = t4 * t377;
            let t617 = t615 * t616;
            let t619 = f64x8::splat(0.0008468139866666666) * t612 * t617;
            let t621 = -f64x8::splat(1.26595) * t588 - t391 - t394 - t399;
            let t624 = f64x8::splat(0.2137) * t182 * t621 * t189;
            let t625 = t5 * t7;
            let t626 = t377 * t182;
            let t629 = f64x8::splat(0.017808333333333332) * t625 * t626 * t190;
            let t630 = t181 * t175;
            let t631 = f64x8::splat(1.0) / t630;
            let t632 = t14 * t631;
            let t635 = f64x8::splat(2.0) * t632 * t190 * t621;
            let t636 = t585 * t614;
            let t637 = t636 * t587;
            let t639 = f64x8::splat(1.0)/((t12).sqrt());
            let t640 = t639 * t2;
            let t641 = t640 * t387;
            let t644 = f64x8::splat(2.5319) * t637 - f64x8::splat(0.204775) * t641 - f64x8::splat(0.08215666666666667) * t390;
            let t645 = t644 * t189;
            let t647 = f64x8::splat(1.0) * t183 * t645;
            let t648 = t181 * t181;
            let t649 = f64x8::splat(1.0) / t648;
            let t650 = t14 * t649;
            let t651 = t178 * t178;
            let t652 = f64x8::splat(1.0) / t651;
            let t653 = t188 * t652;
            let t656 = f64x8::splat(16.081979498692537) * t650 * t653 * t621;
            let t658 = f64x8::splat(4.0) * t406 * t230;
            let t660 = f64x8::splat(4.0) * t410 * t230;
            let t661 = t424 * t229;
            let t664 = -f64x8::splat(2.35315) * t588 - t434 - t435 - t436;
            let t668 = t377 * t201;
            let t672 = t200 * t194;
            let t673 = f64x8::splat(1.0) / t672;
            let t674 = t63 * t673;
            let t681 = f64x8::splat(4.7063) * t637 - f64x8::splat(0.420775) * t641 - f64x8::splat(0.104195) * t390;
            let t682 = t681 * t207;
            let t685 = t200 * t200;
            let t686 = f64x8::splat(1.0) / t685;
            let t687 = t63 * t686;
            let t688 = t197 * t197;
            let t689 = f64x8::splat(1.0) / t688;
            let t690 = t206 * t689;
            let t695 = -f64x8::splat(1.7261666666666666) * t588 - t449 - t450 - t451;
            let t697 = t219 * t695 * t225;
            let t699 = t377 * t219;
            let t703 = t218 * t212;
            let t704 = f64x8::splat(1.0) / t703;
            let t705 = t76 * t704;
            let t706 = t226 * t695;
            let t712 = f64x8::splat(3.4523333333333333) * t637 - f64x8::splat(0.1100325) * t641 - f64x8::splat(0.082785) * t390;
            let t713 = t712 * t225;
            let t716 = t218 * t218;
            let t717 = f64x8::splat(1.0) / t716;
            let t718 = t76 * t717;
            let t719 = t215 * t215;
            let t720 = f64x8::splat(1.0) / t719;
            let t721 = t224 * t720;
            let t722 = t721 * t695;
            let t725 = f64x8::splat(0.20548) * t201 * t664 * t207 - f64x8::splat(0.017123333333333334) * t625 * t668 * t208 - f64x8::splat(2.0) * t674 * t208 * t664 + f64x8::splat(1.0) * t202 * t682 + f64x8::splat(32.16395899738507) * t687 * t690 * t664 - t624 + t629 + t635 - t647 - t656 + f64x8::splat(0.06506148780181044) * t697 - f64x8::splat(0.00542179065015087) * t625 * t699 * t226 - f64x8::splat(1.1696447245269292) * t705 * t706 + f64x8::splat(0.5848223622634646) * t220 * t713 + f64x8::splat(17.315859105681465) * t718 * t722;
            let t726 = t61 * t725;
            let t727 = t41 * t726;
            let t731 = f64x8::splat(0.06506148780181044) * t61 * t697;
            let t732 = t424 * t76;
            let t735 = t61 * t5;
            let t736 = t378 * t236;
            let t738 = f64x8::splat(0.00542179065015087) * t735 * t736;
            let t739 = t704 * t224;
            let t740 = t225 * t695;
            let t741 = t739 * t740;
            let t743 = f64x8::splat(1.1696447245269292) * t234 * t741;
            let t745 = t219 * t712 * t225;
            let t747 = f64x8::splat(0.5848223622634646) * t234 * t745;
            let t748 = t717 * t224;
            let t749 = t720 * t695;
            let t750 = t748 * t749;
            let t752 = f64x8::splat(17.315859105681465) * t234 * t750;
            let t753 = -t594 - f64x8::splat(0.0675260332) * t596 * t598 + f64x8::splat(0.0285764) * t159 * t608 + t619 - t624 + t629 + t635 - t647 - t656 - t658 + t660 - t41 * t661 - t727 - f64x8::splat(0.0021973736767207856) * t424 * t216 + t731 + f64x8::splat(0.5848223622634646) * t732 * t236 - t738 - t743 + t747 + t752;
            let t757 = t244 * t244;
            let t758 = f64x8::splat(1.0) / t757;
            let t759 = t758 * t158;
            let t761 = -f64x8::splat(0.007408333333333334) * t388 - t590;
            let t764 = f64x8::splat(0.0285764) * t759 * t166 * t761;
            let t765 = t245 * t58;
            let t766 = t424 * t166;
            let t771 = t764 + f64x8::splat(0.0675260332) * t765 * t766 - f64x8::splat(0.0285764) * t246 * t607 - t381 - t404 - t408 + t412 - t426 - t459 - t461 + t466 + t470;
            let t774 = f64x8::splat(5.0) * t5 * t11 * t753 - f64x8::splat(45.0) * param_eta * t771 - t581;
            let t775 = t147 * t774;
            let t776 = t146 * t775;
            let t780 = t261 * t513 * t277;
            let t782 = f64x8::splat(0.11557628986739024) * t254 * t780;
            let t783 = t146 * t147;
            let t784 = t269 * t122;
            let t785 = t252 * t784;
            let t786 = t162 * t8;
            let t787 = f64x8::splat(1.0) / t786;
            let t788 = t787 * t277;
            let t791 = f64x8::splat(0.005821825775391099) * t783 * t785 * t788;
            let t792 = -t509 + t516 - t523 - f64x8::splat(0.054878743191129266) * t527 * t531 - f64x8::splat(0.027439371595564633) * t535 * t540 - f64x8::splat(0.043341108700271344) * t549 * t562 - f64x8::splat(0.13002332610081402) * t566 * t568 - f64x8::splat(0.043341108700271344) * t574 * t576 + f64x8::splat(0.043341108700271344) * t776 * t279 - t782 + t791;
            let t795 = t496 + t499 * t792 / f64x8::splat(4.0);
            let t797 = f64x8::splat(1.0) / t287;
            let t799 = t97 * t106 * t795 * t797;
            let t800 = v_rho0 * v_rho0;
            let t802 = f64x8::splat(1.0) / t292 / t800;
            let t803 = v_tau0 * t802;
            let t806 = t415 / f64x8::splat(2.0);
            let t807 = t298 * t806;
            let t810 = -t806;
            let t811 = t308 * t810;
            let t814 = t513 / f64x8::splat(3.0);
            let t815 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t803 * t299 + f64x8::splat(5.0) / f64x8::splat(3.0) * t295 * t807 + f64x8::splat(5.0) / f64x8::splat(3.0) * t305 * t811 + t814;
            let t817 = t320 * t320;
            let t818 = f64x8::splat(1.0) / t817;
            let t819 = t312 * t818;
            let t825 = t317 * t502 / f64x8::splat(3.0);
            let t826 = f64x8::splat(3.0) / f64x8::splat(10.0) * t313 * (f64x8::splat(5.0) / f64x8::splat(3.0) * t807 + f64x8::splat(5.0) / f64x8::splat(3.0) * t811) - t825;
            let t828 = t815 * t321 - t819 * t826;
            let t829 = ((t324).select(f64x8::splat(0.0), t828));
            let t832 = t326 * t326;
            let t833 = f64x8::splat(1.0) / t832;
            let t834 = t325 * t833;
            let t837 = -f64x8::splat(0.64) * t829 * t327 - f64x8::splat(0.64) * t834 * t829;
            let t838 = t837 * t330;
            let t839 = ((t332).select(f64x8::splat(0.0), t828));
            let t841 = t333 * t839;
            let t843 = t335 * t839;
            let t845 = t337 * t839;
            let t847 = t339 * t839;
            let t849 = t341 * t839;
            let t854 = t349 * t349;
            let t855 = f64x8::splat(1.0) / t854;
            let t856 = ((t332).select(t828, f64x8::splat(0.0)));
            let t860 = ((t323).select(t838, (t331).select(-f64x8::splat(0.64) * t839 - f64x8::splat(0.8704) * t841 - f64x8::splat(4.607056813647) * t843 + f64x8::splat(12.2462410087) * t845 - f64x8::splat(9.57855118103) * t847 + f64x8::splat(3.101306810232) * t849 - f64x8::splat(0.362942158544) * t343 * t839, -f64x8::splat(1.05) * t855 * t856 * t352)));
            let t861 = t860 * t374;
            let t862 = t758 * t761;
            let t864 = t357 * t366;
            let t868 = f64x8::splat(1.0) / t364 / t363;
            let t869 = t358 * t868;
            let t870 = t869 * t255;
            let t874 = -f64x8::splat(1.0) * t862 * t864 - f64x8::splat(0.014225094736250906) * t870 * t550 * t513;
            let t875 = f64x8::splat(1.0) / t368;
            let t878 = f64x8::splat(0.0285764) * t862 + f64x8::splat(0.0285764) * t874 * t875;
            let t879 = t878 * t158;
            let t880 = t879 * t166;
            let t881 = t371 * t58;
            let t885 = t880 - f64x8::splat(2.363) * t881 * t766 + t372 * t607 - t381 - t404 - t408 + t412 - t426 - t459 - t461 + t466 + t470 - t484 - t799;
            let t886 = t354 * t885;
            let t887 = t381 + t404 + t408 - t412 + t426 + t459 + t461 - t466 - t470 + t484 + t799 + t861 + t886;
            let tvrho0 = t8 * t887 + t290 - t34 + t375 + t90 + t92;
            acc_vrho_0 = tvrho0;
            let t889 = -t42 - t414;
            let t892 = ((t45).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t48 * t889));
            let t893 = -t889;
            let t896 = ((t52).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t53 * t893));
            let t897 = t892 + t896;
            let t898 = t897 * t60;
            let t899 = t898 * t88;
            let t900 = t41 * t899;
            let t901 = t898 * t86;
            let t902 = f64x8::splat(0.0197516734986138) * t901;
            let t903 = t472 * t889;
            let t905 = ((t45).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t903));
            let t906 = t476 * t893;
            let t908 = ((t52).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t906));
            let t910 = t905 / f64x8::splat(2.0) + t908 / f64x8::splat(2.0);
            let t912 = t97 * t471 * t910;
            let t913 = f64x8::splat(3.0) * t912;
            let t915 = (t381 + t404 - t408 - t412 + t900 + t459 + t902 - t466 - t470) * t108;
            let t917 = t490 * t910;
            let t920 = f64x8::splat(3.0) * t109 * t917 - t915 * t111;
            let t921 = t920 * t113;
            let t923 = t506 * t910;
            let t924 = t529 * t923;
            let t927 = t538 * t921;
            let t928 = t529 * t927;
            let t933 = ((t45).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t99 * t889));
            let t936 = ((t52).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t101 * t893));
            let t938 = t933 / f64x8::splat(2.0) + t936 / f64x8::splat(2.0);
            let t939 = t552 * t938;
            let t940 = t551 * t939;
            let t943 = t552 * t910;
            let t944 = t551 * t943;
            let t948 = t551 * t552 * t921;
            let t951 = t595 * t897;
            let t955 = f64x8::splat(12.0) * t602 + f64x8::splat(12.0) * t605;
            let t956 = t955 * t170;
            let t959 = t898 * t229;
            let t963 = t898 * t76;
            let t966 = -t594 - f64x8::splat(0.0675260332) * t951 * t598 + f64x8::splat(0.0285764) * t159 * t956 + t619 - t624 + t629 + t635 - t647 - t656 + t658 + t660 - t41 * t959 - t727 - f64x8::splat(0.0021973736767207856) * t898 * t216 + t731 + f64x8::splat(0.5848223622634646) * t963 * t236 - t738 - t743 + t747 + t752;
            let t970 = t898 * t166;
            let t975 = t764 + f64x8::splat(0.0675260332) * t765 * t970 - f64x8::splat(0.0285764) * t246 * t955 - t381 - t404 + t408 + t412 - t900 - t459 - t902 + t466 + t470;
            let t978 = f64x8::splat(5.0) * t5 * t11 * t966 - f64x8::splat(45.0) * param_eta * t975 - t581;
            let t979 = t147 * t978;
            let t980 = t146 * t979;
            let t983 = -t509 + t516 - t523 - f64x8::splat(0.054878743191129266) * t527 * t924 - f64x8::splat(0.027439371595564633) * t535 * t928 - f64x8::splat(0.043341108700271344) * t549 * t940 - f64x8::splat(0.13002332610081402) * t566 * t944 - f64x8::splat(0.043341108700271344) * t574 * t948 + f64x8::splat(0.043341108700271344) * t980 * t279 - t782 + t791;
            let t986 = t921 * t285 + t499 * t983 / f64x8::splat(4.0);
            let t989 = t97 * t106 * t986 * t797;
            let t990 = t889 / f64x8::splat(2.0);
            let t991 = t298 * t990;
            let t994 = v_rho1 * v_rho1;
            let t996 = f64x8::splat(1.0) / t302 / t994;
            let t997 = v_tau1 * t996;
            let t1000 = -t990;
            let t1001 = t308 * t1000;
            let t1004 = f64x8::splat(5.0) / f64x8::splat(3.0) * t295 * t991 - f64x8::splat(5.0) / f64x8::splat(3.0) * t997 * t309 + f64x8::splat(5.0) / f64x8::splat(3.0) * t305 * t1001 + t814;
            let t1010 = f64x8::splat(3.0) / f64x8::splat(10.0) * t313 * (f64x8::splat(5.0) / f64x8::splat(3.0) * t991 + f64x8::splat(5.0) / f64x8::splat(3.0) * t1001) - t825;
            let t1012 = t1004 * t321 - t819 * t1010;
            let t1013 = ((t324).select(f64x8::splat(0.0), t1012));
            let t1018 = -f64x8::splat(0.64) * t1013 * t327 - f64x8::splat(0.64) * t834 * t1013;
            let t1019 = t1018 * t330;
            let t1020 = ((t332).select(f64x8::splat(0.0), t1012));
            let t1022 = t333 * t1020;
            let t1024 = t335 * t1020;
            let t1026 = t337 * t1020;
            let t1028 = t339 * t1020;
            let t1030 = t341 * t1020;
            let t1035 = ((t332).select(t1012, f64x8::splat(0.0)));
            let t1039 = ((t323).select(t1019, (t331).select(-f64x8::splat(0.64) * t1020 - f64x8::splat(0.8704) * t1022 - f64x8::splat(4.607056813647) * t1024 + f64x8::splat(12.2462410087) * t1026 - f64x8::splat(9.57855118103) * t1028 + f64x8::splat(3.101306810232) * t1030 - f64x8::splat(0.362942158544) * t343 * t1020, -f64x8::splat(1.05) * t855 * t1035 * t352)));
            let t1040 = t1039 * t374;
            let t1044 = t880 - f64x8::splat(2.363) * t881 * t970 + t372 * t955 - t381 - t404 + t408 + t412 - t900 - t459 - t902 + t466 + t470 - t913 - t989;
            let t1045 = t354 * t1044;
            let t1046 = t381 + t404 - t408 - t412 + t900 + t459 + t902 - t466 - t470 + t913 + t989 + t1040 + t1045;
            let tvrho1 = t8 * t1046 + t290 - t34 + t375 + t90 + t92;
            acc_vrho_1 = tvrho1;
            let t1048 = t97 * t106;
            let t1050 = t528 * t134;
            let t1051 = t120 * t534 * t1050;
            let t1053 = t260 * t263;
            let t1054 = t1053 * t277;
            let t1055 = t259 * t1054;
            let t1056 = t254 * t1055;
            let t1058 = t252 * t269;
            let t1059 = f64x8::splat(1.0) / t162;
            let t1060 = t1059 * t277;
            let t1062 = t783 * t1058 * t1060;
            let t1064 = f64x8::splat(0.027439371595564633) * t1051 + f64x8::splat(0.043341108700271344) * t1056 - f64x8::splat(0.002183184665771662) * t1062;
            let t1065 = t1064 * t797;
            let t1066 = t499 * t1065;
            let t1068 = t1048 * t1066 / f64x8::splat(4.0);
            let t1070 = param_eta * t263;
            let t1072 = -t819 * t1070 - t263 * t321;
            let t1073 = t1072 / f64x8::splat(8.0);
            let t1074 = ((t324).select(f64x8::splat(0.0), t1073));
            let t1079 = -f64x8::splat(0.64) * t1074 * t327 - f64x8::splat(0.64) * t834 * t1074;
            let t1080 = t1079 * t330;
            let t1081 = ((t332).select(f64x8::splat(0.0), t1073));
            let t1083 = t333 * t1081;
            let t1085 = t335 * t1081;
            let t1087 = t337 * t1081;
            let t1089 = t339 * t1081;
            let t1091 = t341 * t1081;
            let t1096 = ((t332).select(t1073, f64x8::splat(0.0)));
            let t1100 = ((t323).select(t1080, (t331).select(-f64x8::splat(0.64) * t1081 - f64x8::splat(0.8704) * t1083 - f64x8::splat(4.607056813647) * t1085 + f64x8::splat(12.2462410087) * t1087 - f64x8::splat(9.57855118103) * t1089 + f64x8::splat(3.101306810232) * t1091 - f64x8::splat(0.362942158544) * t343 * t1081, -f64x8::splat(1.05) * t855 * t1096 * t352)));
            let t1101 = t1100 * t374;
            let t1102 = t869 * t259;
            let t1103 = t875 * t158;
            let t1104 = t1103 * t166;
            let t1106 = t1102 * t1053 * t1104;
            let t1108 = f64x8::splat(0.00015243824895787514) * t1106 - t1068;
            let t1109 = t354 * t1108;
            let tvsigma0 = t8 * (t1068 + t1101 + t1109);
            acc_vsigma_0 = tvsigma0;
            let t1114 = f64x8::splat(0.054878743191129266) * t1051 + f64x8::splat(0.08668221740054269) * t1056 - f64x8::splat(0.004366369331543324) * t1062;
            let t1115 = t1114 * t797;
            let t1116 = t499 * t1115;
            let t1118 = t1048 * t1116 / f64x8::splat(4.0);
            let t1119 = t1072 / f64x8::splat(4.0);
            let t1120 = ((t324).select(f64x8::splat(0.0), t1119));
            let t1125 = -f64x8::splat(0.64) * t1120 * t327 - f64x8::splat(0.64) * t834 * t1120;
            let t1126 = t1125 * t330;
            let t1127 = ((t332).select(f64x8::splat(0.0), t1119));
            let t1129 = t333 * t1127;
            let t1131 = t335 * t1127;
            let t1133 = t337 * t1127;
            let t1135 = t339 * t1127;
            let t1137 = t341 * t1127;
            let t1142 = ((t332).select(t1119, f64x8::splat(0.0)));
            let t1146 = ((t323).select(t1126, (t331).select(-f64x8::splat(0.64) * t1127 - f64x8::splat(0.8704) * t1129 - f64x8::splat(4.607056813647) * t1131 + f64x8::splat(12.2462410087) * t1133 - f64x8::splat(9.57855118103) * t1135 + f64x8::splat(3.101306810232) * t1137 - f64x8::splat(0.362942158544) * t343 * t1127, -f64x8::splat(1.05) * t855 * t1142 * t352)));
            let t1147 = t1146 * t374;
            let t1149 = f64x8::splat(0.0003048764979157503) * t1106 - t1118;
            let t1150 = t354 * t1149;
            let tvsigma1 = t8 * (t1118 + t1147 + t1150);
            acc_vsigma_1 = tvsigma1;
            let tvsigma2 = tvsigma0;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t1152 = t294 * t299;
            let t1153 = t1152 * t321;
            let t1154 = ((t324).select(f64x8::splat(0.0), t1153));
            let t1159 = -f64x8::splat(0.64) * t1154 * t327 - f64x8::splat(0.64) * t834 * t1154;
            let t1160 = t1159 * t330;
            let t1161 = ((t332).select(f64x8::splat(0.0), t1153));
            let t1163 = t333 * t1161;
            let t1165 = t335 * t1161;
            let t1167 = t337 * t1161;
            let t1169 = t339 * t1161;
            let t1171 = t341 * t1161;
            let t1176 = ((t332).select(t1153, f64x8::splat(0.0)));
            let t1180 = ((t323).select(t1160, (t331).select(-f64x8::splat(0.64) * t1161 - f64x8::splat(0.8704) * t1163 - f64x8::splat(4.607056813647) * t1165 + f64x8::splat(12.2462410087) * t1167 - f64x8::splat(9.57855118103) * t1169 + f64x8::splat(3.101306810232) * t1171 - f64x8::splat(0.362942158544) * t343 * t1161, -f64x8::splat(1.05) * t855 * t1176 * t352)));
            let t1181 = t8 * t1180;
            let tvtau0 = t1181 * t374;
            acc_vtau_0 = tvtau0;
            let t1182 = t304 * t309;
            let t1183 = t1182 * t321;
            let t1184 = ((t324).select(f64x8::splat(0.0), t1183));
            let t1189 = -f64x8::splat(0.64) * t1184 * t327 - f64x8::splat(0.64) * t834 * t1184;
            let t1190 = t1189 * t330;
            let t1191 = ((t332).select(f64x8::splat(0.0), t1183));
            let t1193 = t333 * t1191;
            let t1195 = t335 * t1191;
            let t1197 = t337 * t1191;
            let t1199 = t339 * t1191;
            let t1201 = t341 * t1191;
            let t1206 = ((t332).select(t1183, f64x8::splat(0.0)));
            let t1210 = ((t323).select(t1190, (t331).select(-f64x8::splat(0.64) * t1191 - f64x8::splat(0.8704) * t1193 - f64x8::splat(4.607056813647) * t1195 + f64x8::splat(12.2462410087) * t1197 - f64x8::splat(9.57855118103) * t1199 + f64x8::splat(3.101306810232) * t1201 - f64x8::splat(0.362942158544) * t343 * t1191, -f64x8::splat(1.05) * t855 * t1206 * t352)));
            let t1211 = t8 * t1210;
            let tvtau1 = t1211 * t374;
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
