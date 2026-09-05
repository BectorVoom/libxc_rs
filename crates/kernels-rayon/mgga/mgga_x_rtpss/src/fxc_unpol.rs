//! MGGA_X_RTPSS fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rtpss.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_rtpss_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
    param_b: f64,
    param_c: f64,
    param_e: f64,
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_e = f64x8::splat(param_e);
    let param_kappa = f64x8::splat(param_kappa);
    let param_mu = f64x8::splat(param_mu);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2rholapl = V_ZERO;
        let mut acc_v2rhotau = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v2sigmalapl = V_ZERO;
        let mut acc_v2sigmatau = V_ZERO;
        let mut acc_v2lapl2 = V_ZERO;
        let mut acc_v2lapltau = V_ZERO;
        let mut acc_v2tau2 = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = v_sigma * v_sigma;
            let t22 = param_c * t21;
            let t23 = v_rho * v_rho;
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = v_tau * v_tau;
            let t26 = f64x8::splat(1.0) / t25;
            let t27 = t24 * t26;
            let t28 = t21 * t24;
            let t29 = t28 * t26;
            let t31 = f64x8::splat(1.0) + t29 / f64x8::splat(64.0);
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t27 * t33;
            let t38 = f64x8::splat(M_CBRT6);
            let t39 = (f64x8::splat(10.0) / f64x8::splat(81.0) + t22 * t34 / f64x8::splat(64.0)) * t38;
            let t40 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t41 = (simd::cbrt(t40));
            let t42 = t41 * t41;
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = t39 * t43;
            let t45 = f64x8::splat(M_CBRT2);
            let t46 = t45 * t45;
            let t47 = v_sigma * t46;
            let t48 = t19 * t19;
            let t50 = f64x8::splat(1.0) / t48 / t23;
            let t51 = t47 * t50;
            let t54 = v_tau * t46;
            let t56 = f64x8::splat(1.0) / t48 / v_rho;
            let t59 = t54 * t56 - t51 / f64x8::splat(8.0);
            let t63 = f64x8::splat(5.0) / f64x8::splat(9.0) * t59 * t38 * t43 - f64x8::splat(1.0);
            let t64 = param_b * t59;
            let t65 = t38 * t43;
            let t66 = t65 * t63;
            let t69 = f64x8::splat(5.0) * t64 * t66 + f64x8::splat(9.0);
            let t70 = ((t69).sqrt());
            let t71 = f64x8::splat(1.0) / t70;
            let t76 = f64x8::splat(27.0) / f64x8::splat(20.0) * t63 * t71 + t65 * t51 / f64x8::splat(36.0);
            let t77 = t76 * t76;
            let t80 = t38 * t38;
            let t82 = f64x8::splat(1.0) / t41 / t40;
            let t83 = t80 * t82;
            let t84 = t21 * t45;
            let t85 = t23 * t23;
            let t86 = t85 * v_rho;
            let t88 = f64x8::splat(1.0) / t19 / t86;
            let t89 = t84 * t88;
            let t92 = f64x8::splat(100.0) * t83 * t89 + f64x8::splat(162.0) * t29;
            let t93 = ((t92).sqrt());
            let t96 = f64x8::splat(1.0) / param_kappa;
            let t97 = t96 * t80;
            let t98 = t97 * t82;
            let t101 = ((param_e).sqrt());
            let t102 = t101 * t21;
            let t105 = param_e * param_mu;
            let t106 = t40 * t40;
            let t107 = f64x8::splat(1.0) / t106;
            let t108 = t21 * v_sigma;
            let t109 = t107 * t108;
            let t110 = t85 * t85;
            let t111 = f64x8::splat(1.0) / t110;
            let t115 = t44 * t51 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t77 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t76 * t93 + f64x8::splat(25.0) / f64x8::splat(472392.0) * t98 * t89 + t102 * t27 / f64x8::splat(720.0) + t105 * t109 * t111 / f64x8::splat(576.0);
            let t116 = t101 * t38;
            let t120 = f64x8::splat(1.0) + t116 * t43 * t51 / f64x8::splat(24.0);
            let t121 = t120 * t120;
            let t122 = f64x8::splat(1.0) / t121;
            let t125 = (simd::exp(-t115 * t122 * t96));
            let t128 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - t125);
            let t132 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t128));
            let tzk0 = f64x8::splat(2.0) * t132;
            acc_zk = tzk0;
            let t133 = f64x8::splat(1.0) / t48;
            let t134 = t18 * t133;
            let t138 = t7 * t18;
            let t139 = t19 * param_kappa;
            let t140 = t23 * v_rho;
            let t141 = f64x8::splat(1.0) / t140;
            let t142 = t141 * t26;
            let t143 = t142 * t33;
            let t146 = t21 * t21;
            let t147 = param_c * t146;
            let t148 = f64x8::splat(1.0) / t86;
            let t149 = t25 * t25;
            let t150 = f64x8::splat(1.0) / t149;
            let t153 = f64x8::splat(1.0) / t32 / t31;
            let t154 = t148 * t150 * t153;
            let t158 = (-t22 * t143 / f64x8::splat(32.0) + t147 * t154 / f64x8::splat(1024.0)) * t38;
            let t159 = t158 * t43;
            let t163 = f64x8::splat(1.0) / t48 / t140;
            let t164 = t47 * t163;
            let t170 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t54 * t50 + t164 / f64x8::splat(3.0);
            let t171 = t170 * t38;
            let t172 = t43 * t71;
            let t176 = f64x8::splat(1.0) / t70 / t69;
            let t177 = t63 * t176;
            let t181 = t83 * t170;
            let t184 = f64x8::splat(5.0) * param_b * t170 * t66 + f64x8::splat(25.0) / f64x8::splat(9.0) * t64 * t181;
            let t187 = t65 * t164;
            let t189 = f64x8::splat(3.0) / f64x8::splat(4.0) * t171 * t172 - f64x8::splat(27.0) / f64x8::splat(40.0) * t177 * t184 - f64x8::splat(2.0) / f64x8::splat(27.0) * t187;
            let t194 = f64x8::splat(1.0) / t93;
            let t195 = t76 * t194;
            let t196 = t21 * t141;
            let t199 = t85 * t23;
            let t201 = f64x8::splat(1.0) / t19 / t199;
            let t202 = t84 * t201;
            let t205 = -f64x8::splat(324.0) * t196 * t26 - f64x8::splat(1600.0) / f64x8::splat(3.0) * t83 * t202;
            let t212 = t110 * v_rho;
            let t213 = f64x8::splat(1.0) / t212;
            let t217 = t159 * t51 / f64x8::splat(24.0) - t44 * t164 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t76 * t189 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t189 * t93 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t195 * t205 - f64x8::splat(50.0) / f64x8::splat(177147.0) * t98 * t202 - t102 * t142 / f64x8::splat(360.0) - t105 * t109 * t213 / f64x8::splat(72.0);
            let t220 = t121 * t120;
            let t221 = f64x8::splat(1.0) / t220;
            let t223 = t96 * t101;
            let t224 = t115 * t221 * t223;
            let t227 = -t217 * t122 * t96 - f64x8::splat(2.0) / f64x8::splat(9.0) * t224 * t187;
            let t228 = t227 * t125;
            let t233 = ((t3).select(f64x8::splat(0.0), -t7 * t134 * t128 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(8.0) * t138 * t139 * t228));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t233 + f64x8::splat(2.0) * t132;
            acc_vrho = tvrho0;
            let t236 = param_c * v_sigma;
            let t239 = param_c * t108;
            let t240 = f64x8::splat(1.0) / t85;
            let t241 = t240 * t150;
            let t242 = t241 * t153;
            let t246 = (t236 * t34 / f64x8::splat(32.0) - t239 * t242 / f64x8::splat(1024.0)) * t38;
            let t247 = t246 * t43;
            let t250 = t43 * t46;
            let t251 = t250 * t50;
            let t254 = t46 * t50;
            let t255 = t65 * t71;
            let t256 = t254 * t255;
            let t258 = param_b * t46;
            let t259 = t258 * t50;
            let t260 = t259 * t66;
            let t262 = t64 * t80;
            let t263 = t82 * t46;
            let t265 = t262 * t263 * t50;
            let t267 = -f64x8::splat(5.0) / f64x8::splat(8.0) * t260 - f64x8::splat(25.0) / f64x8::splat(72.0) * t265;
            let t270 = t254 * t65;
            let t272 = -f64x8::splat(3.0) / f64x8::splat(32.0) * t256 - f64x8::splat(27.0) / f64x8::splat(40.0) * t177 * t267 + t270 / f64x8::splat(36.0);
            let t277 = v_sigma * t24;
            let t280 = v_sigma * t45;
            let t281 = t280 * t88;
            let t284 = f64x8::splat(324.0) * t277 * t26 + f64x8::splat(200.0) * t83 * t281;
            let t289 = t101 * v_sigma;
            let t292 = t107 * t21;
            let t296 = t247 * t51 / f64x8::splat(24.0) + t39 * t251 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t76 * t272 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t272 * t93 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t195 * t284 + f64x8::splat(25.0) / f64x8::splat(236196.0) * t98 * t281 + t289 * t27 / f64x8::splat(360.0) + t105 * t292 * t111 / f64x8::splat(192.0);
            let t301 = -t296 * t122 * t96 + t224 * t270 / f64x8::splat(12.0);
            let t302 = t301 * t125;
            let t306 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(8.0) * t138 * t139 * t302));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t306;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t308 = t25 * v_tau;
            let t309 = f64x8::splat(1.0) / t308;
            let t310 = t24 * t309;
            let t311 = t310 * t33;
            let t314 = t149 * v_tau;
            let t315 = f64x8::splat(1.0) / t314;
            let t317 = t240 * t315 * t153;
            let t321 = (-t22 * t311 / f64x8::splat(32.0) + t147 * t317 / f64x8::splat(1024.0)) * t38;
            let t322 = t321 * t43;
            let t325 = t46 * t56;
            let t328 = t258 * t56;
            let t334 = f64x8::splat(5.0) * t328 * t66 + f64x8::splat(25.0) / f64x8::splat(9.0) * t262 * t263 * t56;
            let t337 = f64x8::splat(3.0) / f64x8::splat(4.0) * t325 * t255 - f64x8::splat(27.0) / f64x8::splat(40.0) * t177 * t334;
            let t342 = t28 * t309;
            let t347 = t322 * t51 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t76 * t337 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t337 * t93 + f64x8::splat(73.0) / f64x8::splat(600.0) * t195 * t342 - t102 * t310 / f64x8::splat(360.0);
            let t349 = t122 * t125;
            let t353 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t138 * t19 * t347 * t349));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t353;
            acc_vtau = tvtau0;
            let t356 = t18 * t56;
            let t360 = t133 * param_kappa;
            let t364 = t240 * t26;
            let t365 = t364 * t33;
            let t368 = f64x8::splat(1.0) / t199;
            let t370 = t368 * t150 * t153;
            let t373 = t146 * t21;
            let t374 = param_c * t373;
            let t375 = t149 * t25;
            let t376 = f64x8::splat(1.0) / t375;
            let t378 = t32 * t32;
            let t379 = f64x8::splat(1.0) / t378;
            let t380 = t111 * t376 * t379;
            let t384 = (f64x8::splat(3.0) / f64x8::splat(32.0) * t22 * t365 - f64x8::splat(7.0) / f64x8::splat(1024.0) * t147 * t370 + f64x8::splat(3.0) / f64x8::splat(32768.0) * t374 * t380) * t38;
            let t385 = t384 * t43;
            let t391 = f64x8::splat(1.0) / t48 / t85;
            let t392 = t47 * t391;
            let t395 = t189 * t189;
            let t400 = f64x8::splat(40.0) / f64x8::splat(9.0) * t54 * t163 - f64x8::splat(11.0) / f64x8::splat(9.0) * t392;
            let t401 = t400 * t38;
            let t404 = t43 * t176;
            let t405 = t404 * t184;
            let t408 = t69 * t69;
            let t410 = f64x8::splat(1.0) / t70 / t408;
            let t411 = t63 * t410;
            let t412 = t184 * t184;
            let t415 = param_b * t400;
            let t418 = t170 * t170;
            let t422 = t83 * t400;
            let t425 = f64x8::splat(5.0) * t415 * t66 + f64x8::splat(50.0) / f64x8::splat(9.0) * param_b * t418 * t83 + f64x8::splat(25.0) / f64x8::splat(9.0) * t64 * t422;
            let t428 = t65 * t392;
            let t430 = f64x8::splat(3.0) / f64x8::splat(4.0) * t401 * t172 - f64x8::splat(3.0) / f64x8::splat(4.0) * t171 * t405 + f64x8::splat(81.0) / f64x8::splat(80.0) * t411 * t412 - f64x8::splat(27.0) / f64x8::splat(40.0) * t177 * t425 + f64x8::splat(22.0) / f64x8::splat(81.0) * t428;
            let t435 = t189 * t194;
            let t439 = f64x8::splat(1.0) / t93 / t92;
            let t440 = t76 * t439;
            let t441 = t205 * t205;
            let t444 = t21 * t240;
            let t447 = t85 * t140;
            let t449 = f64x8::splat(1.0) / t19 / t447;
            let t450 = t84 * t449;
            let t451 = t83 * t450;
            let t453 = f64x8::splat(972.0) * t444 * t26 + f64x8::splat(30400.0) / f64x8::splat(9.0) * t451;
            let t460 = t110 * t23;
            let t461 = f64x8::splat(1.0) / t460;
            let t465 = t385 * t51 / f64x8::splat(24.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t159 * t164 + f64x8::splat(11.0) / f64x8::splat(27.0) * t44 * t392 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t395 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t76 * t430 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t430 * t93 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t435 * t205 + f64x8::splat(73.0) / f64x8::splat(388800.0) * t440 * t441 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t195 * t453 + f64x8::splat(950.0) / f64x8::splat(531441.0) * t98 * t450 + t102 * t364 / f64x8::splat(120.0) + t105 * t109 * t461 / f64x8::splat(8.0);
            let t469 = t217 * t221 * t223;
            let t472 = t121 * t121;
            let t473 = f64x8::splat(1.0) / t472;
            let t475 = t96 * param_e;
            let t476 = t115 * t473 * t475;
            let t481 = -t465 * t122 * t96 - f64x8::splat(4.0) / f64x8::splat(9.0) * t469 * t187 - f64x8::splat(4.0) / f64x8::splat(27.0) * t476 * t451 + f64x8::splat(22.0) / f64x8::splat(27.0) * t224 * t428;
            let t482 = t481 * t125;
            let t486 = t227 * t227;
            let t487 = t486 * t125;
            let t492 = ((t3).select(f64x8::splat(0.0), t7 * t356 * t128 / f64x8::splat(12.0) + t138 * t360 * t228 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(8.0) * t138 * t139 * t482 + f64x8::splat(3.0) / f64x8::splat(8.0) * t138 * t139 * t487));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t492 + f64x8::splat(4.0) * t233;
            acc_v2rho2 = tv2rho20;
            let t502 = t146 * v_sigma;
            let t503 = param_c * t502;
            let t504 = f64x8::splat(1.0) / t447;
            let t506 = t504 * t376 * t379;
            let t510 = (-t236 * t143 / f64x8::splat(16.0) + f64x8::splat(3.0) / f64x8::splat(512.0) * t239 * t154 - f64x8::splat(3.0) / f64x8::splat(32768.0) * t503 * t506) * t38;
            let t511 = t510 * t43;
            let t518 = t250 * t163;
            let t523 = t46 * t163;
            let t524 = t523 * t255;
            let t526 = t254 * t38;
            let t527 = t526 * t405;
            let t529 = t404 * t267;
            let t532 = t267 * t184;
            let t535 = t258 * t163;
            let t536 = t535 * t66;
            let t538 = t259 * t181;
            let t541 = t262 * t263 * t163;
            let t543 = f64x8::splat(5.0) / f64x8::splat(3.0) * t536 - f64x8::splat(25.0) / f64x8::splat(36.0) * t538 + f64x8::splat(25.0) / f64x8::splat(27.0) * t541;
            let t546 = t523 * t65;
            let t548 = t524 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(64.0) * t527 - f64x8::splat(3.0) / f64x8::splat(8.0) * t171 * t529 + f64x8::splat(81.0) / f64x8::splat(80.0) * t411 * t532 - f64x8::splat(27.0) / f64x8::splat(40.0) * t177 * t543 - f64x8::splat(2.0) / f64x8::splat(27.0) * t546;
            let t553 = t272 * t194;
            let t558 = t284 * t205;
            let t561 = v_sigma * t141;
            let t564 = t280 * t201;
            let t565 = t83 * t564;
            let t567 = -f64x8::splat(648.0) * t561 * t26 - f64x8::splat(3200.0) / f64x8::splat(3.0) * t565;
            let t577 = t511 * t51 / f64x8::splat(24.0) - t247 * t164 / f64x8::splat(9.0) + t158 * t251 / f64x8::splat(24.0) - t39 * t518 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t189 * t272 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t76 * t548 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t548 * t93 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t553 * t205 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t435 * t284 + f64x8::splat(73.0) / f64x8::splat(388800.0) * t440 * t558 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t195 * t567 - f64x8::splat(100.0) / f64x8::splat(177147.0) * t98 * t564 - t289 * t142 / f64x8::splat(180.0) - t105 * t292 * t213 / f64x8::splat(24.0);
            let t581 = t296 * t221 * t223;
            let t590 = -t577 * t122 * t96 - f64x8::splat(2.0) / f64x8::splat(9.0) * t581 * t187 + t469 * t270 / f64x8::splat(12.0) + t476 * t565 / f64x8::splat(18.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t224 * t546;
            let t591 = t590 * t125;
            let t595 = t7 * t20;
            let t596 = param_kappa * t301;
            let t597 = t596 * t228;
            let t601 = ((t3).select(f64x8::splat(0.0), t138 * t360 * t302 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(8.0) * t138 * t139 * t591 + f64x8::splat(3.0) / f64x8::splat(8.0) * t595 * t597));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t601 + f64x8::splat(2.0) * t306;
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t608 = t141 * t309;
            let t609 = t608 * t33;
            let t613 = t148 * t315 * t153;
            let t617 = f64x8::splat(1.0) / t149 / t308;
            let t619 = t504 * t617 * t379;
            let t623 = (t22 * t609 / f64x8::splat(16.0) - f64x8::splat(3.0) / f64x8::splat(512.0) * t147 * t613 + f64x8::splat(3.0) / f64x8::splat(32768.0) * t374 * t619) * t38;
            let t624 = t623 * t43;
            let t632 = t325 * t38;
            let t635 = t404 * t334;
            let t638 = t334 * t184;
            let t645 = -f64x8::splat(25.0) / f64x8::splat(3.0) * t260 + f64x8::splat(50.0) / f64x8::splat(9.0) * t328 * t181 - f64x8::splat(125.0) / f64x8::splat(27.0) * t265;
            let t648 = -f64x8::splat(5.0) / f64x8::splat(4.0) * t256 - f64x8::splat(3.0) / f64x8::splat(8.0) * t632 * t405 - f64x8::splat(3.0) / f64x8::splat(8.0) * t171 * t635 + f64x8::splat(81.0) / f64x8::splat(80.0) * t411 * t638 - f64x8::splat(27.0) / f64x8::splat(40.0) * t177 * t645;
            let t653 = t337 * t194;
            let t658 = t440 * t21;
            let t659 = t310 * t205;
            let t662 = t196 * t309;
            let t667 = t624 * t51 / f64x8::splat(24.0) - t322 * t164 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t189 * t337 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t76 * t648 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t648 * t93 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t653 * t205 + f64x8::splat(73.0) / f64x8::splat(600.0) * t435 * t342 - f64x8::splat(73.0) / f64x8::splat(1200.0) * t658 * t659 - f64x8::splat(73.0) / f64x8::splat(300.0) * t195 * t662 + t102 * t608 / f64x8::splat(180.0);
            let t673 = f64x8::splat(1.0) / t19 / t140;
            let t674 = t673 * t347;
            let t676 = t138 * t674 * t221;
            let t680 = t43 * v_sigma * t46;
            let t681 = t125 * t101 * t38 * t680;
            let t684 = t347 * t122;
            let t685 = t684 * t228;
            let t689 = ((t3).select(f64x8::splat(0.0), -t138 * t133 * t347 * t349 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t138 * t19 * t667 * t349 - t676 * t681 / f64x8::splat(12.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t595 * t685));
            let tv2rhotau0 = f64x8::splat(2.0) * v_rho * t689 + f64x8::splat(2.0) * t353;
            acc_v2rhotau = tv2rhotau0;
            let t692 = param_c * t24;
            let t693 = t26 * t33;
            let t698 = t368 * t376;
            let t699 = t698 * t379;
            let t703 = (t692 * t693 / f64x8::splat(32.0) - f64x8::splat(5.0) / f64x8::splat(1024.0) * t22 * t242 + f64x8::splat(3.0) / f64x8::splat(32768.0) * t147 * t699) * t38;
            let t704 = t703 * t43;
            let t709 = t272 * t272;
            let t711 = t526 * t529;
            let t713 = t267 * t267;
            let t716 = t177 * param_b;
            let t717 = t45 * t88;
            let t718 = t83 * t717;
            let t719 = t716 * t718;
            let t721 = f64x8::splat(3.0) / f64x8::splat(32.0) * t711 + f64x8::splat(81.0) / f64x8::splat(80.0) * t411 * t713 - f64x8::splat(15.0) / f64x8::splat(128.0) * t719;
            let t728 = t284 * t284;
            let t733 = f64x8::splat(324.0) * t27 + f64x8::splat(200.0) * t718;
            let t736 = t82 * t45;
            let t740 = t101 * t24;
            let t743 = t107 * v_sigma;
            let t747 = t704 * t51 / f64x8::splat(24.0) + t246 * t251 / f64x8::splat(12.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t709 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t76 * t721 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t721 * t93 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t553 * t284 + f64x8::splat(73.0) / f64x8::splat(388800.0) * t440 * t728 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t195 * t733 + f64x8::splat(25.0) / f64x8::splat(236196.0) * t97 * t736 * t88 + t740 * t26 / f64x8::splat(360.0) + t105 * t743 * t111 / f64x8::splat(96.0);
            let t754 = -t747 * t122 * t96 + t581 * t270 / f64x8::splat(6.0) - t476 * t718 / f64x8::splat(48.0);
            let t755 = t754 * t125;
            let t758 = t301 * t301;
            let t759 = t758 * t125;
            let t764 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(8.0) * t138 * t139 * t755 + f64x8::splat(3.0) / f64x8::splat(8.0) * t138 * t139 * t759));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t764;
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t771 = t368 * t617 * t379;
            let t775 = (-t236 * t311 / f64x8::splat(16.0) + f64x8::splat(3.0) / f64x8::splat(512.0) * t239 * t317 - f64x8::splat(3.0) / f64x8::splat(32768.0) * t503 * t771) * t38;
            let t776 = t775 * t43;
            let t785 = t526 * t635;
            let t787 = t334 * t267;
            let t791 = f64x8::splat(1.0) / t19 / t85;
            let t792 = t45 * t791;
            let t793 = t792 * t83;
            let t794 = t716 * t793;
            let t796 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t632 * t529 + f64x8::splat(3.0) / f64x8::splat(64.0) * t785 + f64x8::splat(81.0) / f64x8::splat(80.0) * t411 * t787 + f64x8::splat(15.0) / f64x8::splat(16.0) * t794;
            let t805 = t310 * t284;
            let t808 = t277 * t309;
            let t813 = t776 * t51 / f64x8::splat(24.0) + t321 * t251 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t272 * t337 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t76 * t796 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t796 * t93 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t653 * t284 + f64x8::splat(73.0) / f64x8::splat(600.0) * t553 * t342 - f64x8::splat(73.0) / f64x8::splat(1200.0) * t658 * t805 + f64x8::splat(73.0) / f64x8::splat(300.0) * t195 * t808 - t289 * t310 / f64x8::splat(180.0);
            let t819 = f64x8::splat(1.0) / t19 / t23;
            let t820 = t18 * t819;
            let t825 = t65 * t46;
            let t826 = t221 * t125 * t101 * t825;
            let t829 = t684 * t302;
            let t833 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t138 * t19 * t813 * t349 + t7 * t820 * t347 * t826 / f64x8::splat(32.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t595 * t829));
            let tv2sigmatau0 = f64x8::splat(2.0) * v_rho * t833;
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let t835 = t24 * t150;
            let t836 = t835 * t33;
            let t839 = t240 * t376;
            let t840 = t839 * t153;
            let t843 = t149 * t149;
            let t844 = f64x8::splat(1.0) / t843;
            let t846 = t368 * t844 * t379;
            let t850 = (f64x8::splat(3.0) / f64x8::splat(32.0) * t22 * t836 - f64x8::splat(7.0) / f64x8::splat(1024.0) * t147 * t840 + f64x8::splat(3.0) / f64x8::splat(32768.0) * t374 * t846) * t38;
            let t851 = t850 * t43;
            let t854 = t337 * t337;
            let t858 = t334 * t334;
            let t861 = t45 * t673;
            let t862 = t861 * t83;
            let t865 = -f64x8::splat(3.0) / f64x8::splat(4.0) * t632 * t635 + f64x8::splat(81.0) / f64x8::splat(80.0) * t411 * t858 - f64x8::splat(15.0) / f64x8::splat(2.0) * t716 * t862;
            let t872 = t146 * t240;
            let t873 = t872 * t376;
            let t876 = t28 * t150;
            let t881 = t851 * t51 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t854 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t76 * t865 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t865 * t93 + f64x8::splat(73.0) / f64x8::splat(300.0) * t653 * t342 + f64x8::splat(1971.0) / f64x8::splat(100.0) * t440 * t873 - f64x8::splat(73.0) / f64x8::splat(200.0) * t195 * t876 + t102 * t835 / f64x8::splat(120.0);
            let t885 = t347 * t347;
            let t886 = t885 * t473;
            let t887 = t96 * t125;
            let t888 = t886 * t887;
            let t892 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t138 * t19 * t881 * t349 + f64x8::splat(3.0) / f64x8::splat(8.0) * t595 * t888));
            let tv2tau20 = f64x8::splat(2.0) * v_rho * t892;
            acc_v2tau2 = tv2tau20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2rholapl, ip, m, acc_v2rholapl);
        store_add(v2rhotau, ip, m, acc_v2rhotau);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v2sigmalapl, ip, m, acc_v2sigmalapl);
        store_add(v2sigmatau, ip, m, acc_v2sigmatau);
        store_add(v2lapl2, ip, m, acc_v2lapl2);
        store_add(v2lapltau, ip, m, acc_v2lapltau);
        store_add(v2tau2, ip, m, acc_v2tau2);
        ip += 8;
    }
}
