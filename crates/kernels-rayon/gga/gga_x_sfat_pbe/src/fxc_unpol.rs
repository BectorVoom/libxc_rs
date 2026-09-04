//! GGA_X_SFAT_PBE fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sfat_pbe.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_sfat_pbe_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = t17 / t4 * t3;
            let t19 = (simd::cbrt(v_rho));
            let t20 = t3 * t3;
            let t22 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = f64x8::splat(M_CBRT4);
            let t27 = f64x8::splat(M_CBRT6);
            let t28 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t29 = (simd::cbrt(t28));
            let t30 = t29 * t29;
            let t31 = f64x8::splat(1.0) / t30;
            let t32 = t31 * t27;
            let t33 = f64x8::splat(M_CBRT2);
            let t34 = t33 * t33;
            let t35 = t34 * v_sigma;
            let t36 = v_rho * v_rho;
            let t37 = t19 * t19;
            let t39 = f64x8::splat(1.0) / t37 / t36;
            let t43 = f64x8::splat(0.804) + f64x8::splat(0.009146457198521547) * t39 * t35 * t32;
            let t46 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t43;
            let t49 = f64x8::splat(1.0) / t46 * t25 * t24 * t20 * f64x8::splat(M_PI);
            let t50 = ((t49).sqrt());
            let t52 = f64x8::splat(1.0) / t50 * param_hyb_omega_0;
            let t53 = v_rho * t11;
            let t54 = (simd::cbrt(t53));
            let t55 = f64x8::splat(1.0) / t54;
            let t58 = t55 * t33 * t52 / f64x8::splat(2.0);
            let t59 = (f64x8::splat(1.92)).simd_le(t58);
            let t60 = (f64x8::splat(1.92)).simd_lt(t58);
            let t61 = ((t60).select(t58, f64x8::splat(1.92)));
            let t62 = t61 * t61;
            let t63 = t62 * t62;
            let t64 = f64x8::splat(1.0) / t63;
            let t66 = t63 * t62;
            let t67 = f64x8::splat(1.0) / t66;
            let t69 = t63 * t63;
            let t70 = f64x8::splat(1.0) / t69;
            let t72 = t69 * t62;
            let t73 = f64x8::splat(1.0) / t72;
            let t75 = t69 * t63;
            let t76 = f64x8::splat(1.0) / t75;
            let t78 = t69 * t66;
            let t79 = f64x8::splat(1.0) / t78;
            let t81 = t69 * t69;
            let t82 = f64x8::splat(1.0) / t81;
            let t85 = f64x8::splat(1.0) / t81 / t62;
            let t88 = f64x8::splat(1.0) / t81 / t63;
            let t91 = f64x8::splat(1.0) / t81 / t66;
            let t94 = f64x8::splat(1.0) / t81 / t69;
            let t97 = f64x8::splat(1.0) / t81 / t72;
            let t100 = f64x8::splat(1.0) / t81 / t75;
            let t103 = f64x8::splat(1.0) / t81 / t78;
            let t105 = t81 * t81;
            let t106 = f64x8::splat(1.0) / t105;
            let t109 = f64x8::splat(1.0) / t105 / t62;
            let t112 = f64x8::splat(1.0) / t105 / t63;
            let t116 = -t64 / f64x8::splat(30.0) + t67 / f64x8::splat(70.0) - t70 / f64x8::splat(135.0) + t73 / f64x8::splat(231.0) - t76 / f64x8::splat(364.0) + t79 / f64x8::splat(540.0) - t82 / f64x8::splat(765.0) + t85 / f64x8::splat(1045.0) - t88 / f64x8::splat(1386.0) + t91 / f64x8::splat(1794.0) - t94 / f64x8::splat(2275.0) + t97 / f64x8::splat(2835.0) - t100 / f64x8::splat(3480.0) + t103 / f64x8::splat(4216.0) - t106 / f64x8::splat(5049.0) + t109 / f64x8::splat(5985.0) - t112 / f64x8::splat(7030.0) + f64x8::splat(1.0) / t62 / f64x8::splat(9.0);
            let t117 = ((t60).select(f64x8::splat(1.92), t58));
            let t118 = (simd::atan2(f64x8::splat(1.0), t117));
            let t119 = t117 * t117;
            let t120 = t119 + f64x8::splat(3.0);
            let t121 = f64x8::splat(1.0) / t119;
            let t122 = f64x8::splat(1.0) + t121;
            let t123 = (simd::ln(t122));
            let t125 = -t123 * t120 + f64x8::splat(1.0);
            let t128 = t118 + t125 * t117 / f64x8::splat(4.0);
            let t132 = ((t59).select(t116, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t128 * t117));
            let t137 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t46 * t132 * t19 * t18));
            let tzk0 = f64x8::splat(2.0) * t137;
            acc_zk = tzk0;
            let t138 = f64x8::splat(1.0) / t37;
            let t143 = t63 * t61;
            let t144 = f64x8::splat(1.0) / t143;
            let t147 = f64x8::splat(1.0) / t50 / t49 * param_hyb_omega_0;
            let t149 = t24 * t20;
            let t150 = t25 * t149;
            let t151 = t150 * t55 * t147;
            let t152 = t46 * t46;
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t43 * t43;
            let t155 = f64x8::splat(1.0) / t154;
            let t157 = t27 * t155 * t153;
            let t158 = v_sigma * t31;
            let t159 = t36 * v_rho;
            let t161 = f64x8::splat(1.0) / t37 / t159;
            let t167 = f64x8::splat(1.0) / t54 / t53;
            let t172 = -f64x8::splat(0.02476587138536942) * t161 * t158 * t157 * t151 - t11 * t167 * t33 * t52 / f64x8::splat(6.0);
            let t173 = ((t60).select(t172, f64x8::splat(0.0)));
            let t176 = t62 * t61;
            let t177 = t63 * t176;
            let t178 = f64x8::splat(1.0) / t177;
            let t181 = t69 * t61;
            let t182 = f64x8::splat(1.0) / t181;
            let t185 = t69 * t176;
            let t186 = f64x8::splat(1.0) / t185;
            let t189 = t69 * t143;
            let t190 = f64x8::splat(1.0) / t189;
            let t193 = t69 * t177;
            let t194 = f64x8::splat(1.0) / t193;
            let t198 = f64x8::splat(1.0) / t81 / t61;
            let t202 = f64x8::splat(1.0) / t81 / t176;
            let t206 = f64x8::splat(1.0) / t81 / t143;
            let t210 = f64x8::splat(1.0) / t81 / t177;
            let t214 = f64x8::splat(1.0) / t81 / t181;
            let t218 = f64x8::splat(1.0) / t81 / t185;
            let t222 = f64x8::splat(1.0) / t81 / t189;
            let t226 = f64x8::splat(1.0) / t81 / t193;
            let t230 = f64x8::splat(1.0) / t105 / t61;
            let t234 = f64x8::splat(1.0) / t105 / t176;
            let t238 = f64x8::splat(1.0) / t105 / t143;
            let t241 = f64x8::splat(1.0) / t176;
            let t244 = f64x8::splat(2.0) / f64x8::splat(15.0) * t173 * t144 - f64x8::splat(3.0) / f64x8::splat(35.0) * t173 * t178 + f64x8::splat(8.0) / f64x8::splat(135.0) * t173 * t182 - f64x8::splat(10.0) / f64x8::splat(231.0) * t173 * t186 + f64x8::splat(3.0) / f64x8::splat(91.0) * t173 * t190 - f64x8::splat(7.0) / f64x8::splat(270.0) * t173 * t194 + f64x8::splat(16.0) / f64x8::splat(765.0) * t173 * t198 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t173 * t202 + f64x8::splat(10.0) / f64x8::splat(693.0) * t173 * t206 - f64x8::splat(11.0) / f64x8::splat(897.0) * t173 * t210 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t173 * t214 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t173 * t218 + f64x8::splat(7.0) / f64x8::splat(870.0) * t173 * t222 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t173 * t226 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t173 * t230 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t173 * t234 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t173 * t238 - f64x8::splat(2.0) / f64x8::splat(9.0) * t173 * t241;
            let t245 = ((t60).select(f64x8::splat(0.0), t172));
            let t248 = f64x8::splat(1.0) / t122;
            let t254 = t119 * t117;
            let t255 = f64x8::splat(1.0) / t254;
            let t256 = t255 * t120;
            let t257 = t248 * t245;
            let t260 = -f64x8::splat(2.0) * t123 * t245 * t117 + f64x8::splat(2.0) * t257 * t256;
            let t263 = -t248 * t121 * t245 + t125 * t245 / f64x8::splat(4.0) + t260 * t117 / f64x8::splat(4.0);
            let t267 = ((t59).select(t244, -f64x8::splat(8.0) / f64x8::splat(3.0) * t263 * t117 - f64x8::splat(8.0) / f64x8::splat(3.0) * t128 * t245));
            let t272 = t17 * t3;
            let t274 = f64x8::splat(1.0) / t19 / t159;
            let t276 = t132 * t274 * t272;
            let t277 = t27 * t155;
            let t278 = t34 * t158;
            let t279 = t278 * t277;
            let t283 = ((t2).select(f64x8::splat(0.0), -t46 * t132 * t138 * t18 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t46 * t267 * t19 * t18 + f64x8::splat(0.0040369036088841095) * t279 * t276));
            let tvrho0 = f64x8::splat(2.0) * t283 * v_rho + f64x8::splat(2.0) * t137;
            acc_vrho = tvrho0;
            let t288 = t24 * t20 * t55 * t147;
            let t289 = t153 * t25;
            let t290 = t155 * t289;
            let t294 = f64x8::splat(0.009287201769513533) * t39 * t32 * t290 * t288;
            let t295 = ((t60).select(t294, f64x8::splat(0.0)));
            let t296 = t295 * t144;
            let t298 = t295 * t178;
            let t300 = t295 * t182;
            let t302 = t295 * t186;
            let t304 = t295 * t190;
            let t306 = t295 * t194;
            let t308 = t295 * t198;
            let t310 = t295 * t202;
            let t312 = t295 * t206;
            let t314 = t295 * t210;
            let t316 = t295 * t214;
            let t318 = t295 * t218;
            let t320 = t295 * t222;
            let t322 = t295 * t226;
            let t324 = t295 * t230;
            let t326 = t295 * t234;
            let t328 = t295 * t238;
            let t332 = f64x8::splat(2.0) / f64x8::splat(15.0) * t296 - f64x8::splat(3.0) / f64x8::splat(35.0) * t298 + f64x8::splat(8.0) / f64x8::splat(135.0) * t300 - f64x8::splat(10.0) / f64x8::splat(231.0) * t302 + f64x8::splat(3.0) / f64x8::splat(91.0) * t304 - f64x8::splat(7.0) / f64x8::splat(270.0) * t306 + f64x8::splat(16.0) / f64x8::splat(765.0) * t308 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t310 + f64x8::splat(10.0) / f64x8::splat(693.0) * t312 - f64x8::splat(11.0) / f64x8::splat(897.0) * t314 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t316 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t318 + f64x8::splat(7.0) / f64x8::splat(870.0) * t320 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t322 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t324 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t326 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t328 - f64x8::splat(2.0) / f64x8::splat(9.0) * t295 * t241;
            let t333 = ((t60).select(f64x8::splat(0.0), t294));
            let t335 = t121 * t333;
            let t341 = t248 * t333;
            let t344 = -f64x8::splat(2.0) * t123 * t333 * t117 + f64x8::splat(2.0) * t341 * t256;
            let t347 = -t248 * t335 + t125 * t333 / f64x8::splat(4.0) + t344 * t117 / f64x8::splat(4.0);
            let t351 = ((t59).select(t332, -f64x8::splat(8.0) / f64x8::splat(3.0) * t347 * t117 - f64x8::splat(8.0) / f64x8::splat(3.0) * t128 * t333));
            let t357 = f64x8::splat(1.0) / t19 / t36;
            let t360 = t34 * t31;
            let t361 = t360 * t277;
            let t365 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t46 * t351 * t19 * t18 - f64x8::splat(0.0015138388533315413) * t361 * t132 * t357 * t272));
            let tvsigma0 = f64x8::splat(2.0) * t365 * v_rho;
            acc_vsigma = tvsigma0;
            let t369 = f64x8::splat(1.0) / t37 / v_rho;
            let t378 = t36 * t36;
            let t380 = f64x8::splat(1.0) / t19 / t378;
            let t382 = t132 * t380 * t272;
            let t385 = t173 * t173;
            let t391 = f64x8::splat(1.0) / t105 / t66;
            let t424 = -f64x8::splat(32.0) / f64x8::splat(153.0) * t385 * t109 + f64x8::splat(34.0) / f64x8::splat(171.0) * t385 * t112 - f64x8::splat(18.0) / f64x8::splat(95.0) * t385 * t391 + f64x8::splat(2.0) / f64x8::splat(3.0) * t385 * t64 + f64x8::splat(26.0) / f64x8::splat(105.0) * t385 * t100 - f64x8::splat(7.0) / f64x8::splat(30.0) * t385 * t103 + f64x8::splat(15.0) / f64x8::splat(68.0) * t385 * t106 + f64x8::splat(18.0) / f64x8::splat(55.0) * t385 * t88 - f64x8::splat(10.0) / f64x8::splat(33.0) * t385 * t91 + f64x8::splat(11.0) / f64x8::splat(39.0) * t385 * t94 - f64x8::splat(24.0) / f64x8::splat(91.0) * t385 * t97 - f64x8::splat(3.0) / f64x8::splat(7.0) * t385 * t79 + f64x8::splat(7.0) / f64x8::splat(18.0) * t385 * t82 - f64x8::splat(16.0) / f64x8::splat(45.0) * t385 * t85 - f64x8::splat(2.0) / f64x8::splat(3.0) * t385 * t67 + f64x8::splat(3.0) / f64x8::splat(5.0) * t385 * t70 - f64x8::splat(8.0) / f64x8::splat(15.0) * t385 * t73 + f64x8::splat(10.0) / f64x8::splat(21.0) * t385 * t76;
            let t426 = t23 * t23;
            let t427 = f64x8::splat(1.0) / t426;
            let t428 = t25 * t25;
            let t435 = f64x8::splat(1.0) / t50 / t153 / t428 / t427 / t3 / t28 * param_hyb_omega_0 / f64x8::splat(3.0);
            let t437 = t427 * t3;
            let t438 = t428 * t437;
            let t439 = t438 * t55 * t435;
            let t440 = t152 * t152;
            let t441 = f64x8::splat(1.0) / t440;
            let t442 = t154 * t154;
            let t443 = f64x8::splat(1.0) / t442;
            let t445 = t27 * t27;
            let t446 = t445 * t443 * t441;
            let t448 = f64x8::splat(1.0) / t29 / t28;
            let t449 = v_sigma * v_sigma;
            let t450 = t449 * t448;
            let t451 = t378 * t159;
            let t453 = f64x8::splat(1.0) / t19 / t451;
            let t455 = t34 * t453 * t450;
            let t459 = t167 * t147;
            let t460 = t150 * t459;
            let t466 = t152 * t46;
            let t467 = f64x8::splat(1.0) / t466;
            let t469 = t445 * t443 * t467;
            let t473 = t154 * t43;
            let t474 = f64x8::splat(1.0) / t473;
            let t476 = t445 * t474 * t153;
            let t481 = f64x8::splat(1.0) / t37 / t378;
            let t486 = t11 * t11;
            let t489 = f64x8::splat(1.0) / t54 / t36 / t486;
            let t494 = f64x8::splat(0.005520135469289938) * t455 * t446 * t439 + f64x8::splat(0.016510580923579612) * t11 * t161 * t158 * t157 * t460 - f64x8::splat(0.0007809394190883494) * t455 * t469 * t151 - f64x8::splat(0.001208106573921978) * t455 * t476 * t151 + f64x8::splat(0.09080819507968788) * t481 * t158 * t157 * t151 + f64x8::splat(2.0) / f64x8::splat(9.0) * t486 * t489 * t33 * t52;
            let t495 = ((t60).select(t494, f64x8::splat(0.0)));
            let t532 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t495 * t241 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t495 * t238 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t495 * t234 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t495 * t230 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t495 * t226 + f64x8::splat(7.0) / f64x8::splat(870.0) * t495 * t222 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t495 * t218 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t495 * t214 + f64x8::splat(10.0) / f64x8::splat(693.0) * t495 * t206 - f64x8::splat(11.0) / f64x8::splat(897.0) * t495 * t210 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t495 * t202 + f64x8::splat(16.0) / f64x8::splat(765.0) * t495 * t198 - f64x8::splat(7.0) / f64x8::splat(270.0) * t495 * t194 + f64x8::splat(3.0) / f64x8::splat(91.0) * t495 * t190 - f64x8::splat(10.0) / f64x8::splat(231.0) * t495 * t186 + f64x8::splat(8.0) / f64x8::splat(135.0) * t495 * t182 - f64x8::splat(3.0) / f64x8::splat(35.0) * t495 * t178 + f64x8::splat(2.0) / f64x8::splat(15.0) * t495 * t144;
            let t534 = ((t60).select(f64x8::splat(0.0), t494));
            let t539 = t121 * t534;
            let t541 = t245 * t245;
            let t542 = t255 * t541;
            let t545 = t119 * t119;
            let t547 = f64x8::splat(1.0) / t545 / t117;
            let t549 = t122 * t122;
            let t550 = f64x8::splat(1.0) / t549;
            let t565 = f64x8::splat(1.0) / t545;
            let t566 = t565 * t120;
            let t567 = t248 * t541;
            let t570 = t248 * t534;
            let t574 = f64x8::splat(1.0) / t545 / t119;
            let t575 = t574 * t120;
            let t576 = t550 * t541;
            let t579 = -f64x8::splat(2.0) * t123 * t534 * t117 + f64x8::splat(8.0) * t248 * t541 * t121 - f64x8::splat(2.0) * t123 * t541 + f64x8::splat(2.0) * t570 * t256 - f64x8::splat(6.0) * t567 * t566 + f64x8::splat(4.0) * t576 * t575;
            let t582 = -t248 * t539 + f64x8::splat(2.0) * t248 * t542 - f64x8::splat(2.0) * t550 * t547 * t541 + t125 * t534 / f64x8::splat(4.0) + t260 * t245 / f64x8::splat(2.0) + t579 * t117 / f64x8::splat(4.0);
            let t586 = ((t59).select(t424 + t532, -f64x8::splat(8.0) / f64x8::splat(3.0) * t128 * t534 - f64x8::splat(16.0) / f64x8::splat(3.0) * t263 * t245 - f64x8::splat(8.0) / f64x8::splat(3.0) * t582 * t117));
            let t592 = t267 * t274 * t272;
            let t595 = f64x8::splat(1.0) / t451;
            let t597 = t132 * t595 * t272;
            let t598 = t445 * t474;
            let t600 = t33 * t450 * t598;
            let t604 = ((t2).select(f64x8::splat(0.0), t46 * t132 * t369 * t18 / f64x8::splat(12.0) - t46 * t267 * t138 * t18 / f64x8::splat(4.0) - f64x8::splat(0.01211071082665233) * t279 * t382 - f64x8::splat(3.0) / f64x8::splat(8.0) * t46 * t586 * t19 * t18 + f64x8::splat(0.008073807217768219) * t279 * t592 + f64x8::splat(0.0003938492381143005) * t600 * t597));
            let tv2rho20 = f64x8::splat(2.0) * t604 * v_rho + f64x8::splat(4.0) * t283;
            acc_v2rho2 = tv2rho20;
            let t611 = t378 * t36;
            let t613 = f64x8::splat(1.0) / t19 / t611;
            let t614 = t613 * t448;
            let t615 = t35 * t614;
            let t619 = t39 * t31;
            let t634 = -f64x8::splat(0.0020700508009837268) * t615 * t446 * t439 - f64x8::splat(0.0030957339231711773) * t11 * t619 * t157 * t460 + f64x8::splat(0.00029285228215813103) * t615 * t469 * t151 + f64x8::splat(0.0004530399652207418) * t615 * t476 * t151 - f64x8::splat(0.02476587138536942) * t161 * t32 * t290 * t288;
            let t635 = ((t60).select(t634, f64x8::splat(0.0)));
            let t636 = t635 * t238;
            let t640 = t635 * t234;
            let t642 = t635 * t230;
            let t644 = t635 * t222;
            let t646 = t635 * t226;
            let t648 = t635 * t218;
            let t650 = t635 * t210;
            let t652 = t635 * t214;
            let t654 = t635 * t206;
            let t656 = t635 * t198;
            let t658 = t635 * t202;
            let t660 = t635 * t194;
            let t662 = t635 * t186;
            let t664 = t635 * t190;
            let t666 = t635 * t182;
            let t668 = t635 * t144;
            let t670 = t635 * t178;
            let t672 = f64x8::splat(18.0) / f64x8::splat(3515.0) * t636 - f64x8::splat(2.0) / f64x8::splat(9.0) * t635 * t241 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t640 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t642 + f64x8::splat(7.0) / f64x8::splat(870.0) * t644 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t646 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t648 - f64x8::splat(11.0) / f64x8::splat(897.0) * t650 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t652 + f64x8::splat(10.0) / f64x8::splat(693.0) * t654 + f64x8::splat(16.0) / f64x8::splat(765.0) * t656 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t658 - f64x8::splat(7.0) / f64x8::splat(270.0) * t660 - f64x8::splat(10.0) / f64x8::splat(231.0) * t662 + f64x8::splat(3.0) / f64x8::splat(91.0) * t664 + f64x8::splat(8.0) / f64x8::splat(135.0) * t666 + f64x8::splat(2.0) / f64x8::splat(15.0) * t668 - f64x8::splat(3.0) / f64x8::splat(35.0) * t670;
            let t673 = t295 * t109;
            let t676 = t295 * t112;
            let t679 = t295 * t391;
            let t682 = t295 * t64;
            let t685 = t295 * t94;
            let t688 = t295 * t97;
            let t691 = t295 * t100;
            let t694 = t295 * t103;
            let t697 = t295 * t106;
            let t700 = t295 * t82;
            let t703 = t295 * t85;
            let t706 = t295 * t88;
            let t709 = t295 * t91;
            let t712 = t295 * t67;
            let t715 = t295 * t70;
            let t718 = t295 * t73;
            let t721 = t295 * t76;
            let t724 = t295 * t79;
            let t727 = -f64x8::splat(32.0) / f64x8::splat(153.0) * t173 * t673 + f64x8::splat(34.0) / f64x8::splat(171.0) * t173 * t676 - f64x8::splat(18.0) / f64x8::splat(95.0) * t173 * t679 + f64x8::splat(2.0) / f64x8::splat(3.0) * t173 * t682 + f64x8::splat(11.0) / f64x8::splat(39.0) * t173 * t685 - f64x8::splat(24.0) / f64x8::splat(91.0) * t173 * t688 + f64x8::splat(26.0) / f64x8::splat(105.0) * t173 * t691 - f64x8::splat(7.0) / f64x8::splat(30.0) * t173 * t694 + f64x8::splat(15.0) / f64x8::splat(68.0) * t173 * t697 + f64x8::splat(7.0) / f64x8::splat(18.0) * t173 * t700 - f64x8::splat(16.0) / f64x8::splat(45.0) * t173 * t703 + f64x8::splat(18.0) / f64x8::splat(55.0) * t173 * t706 - f64x8::splat(10.0) / f64x8::splat(33.0) * t173 * t709 - f64x8::splat(2.0) / f64x8::splat(3.0) * t173 * t712 + f64x8::splat(3.0) / f64x8::splat(5.0) * t173 * t715 - f64x8::splat(8.0) / f64x8::splat(15.0) * t173 * t718 + f64x8::splat(10.0) / f64x8::splat(21.0) * t173 * t721 - f64x8::splat(3.0) / f64x8::splat(7.0) * t173 * t724;
            let t729 = ((t60).select(f64x8::splat(0.0), t634));
            let t733 = t121 * t729;
            let t735 = t255 * t333;
            let t738 = t547 * t333;
            let t739 = t245 * t550;
            let t756 = t245 * t341;
            let t759 = t248 * t729;
            let t762 = t550 * t333;
            let t763 = t245 * t762;
            let t766 = -f64x8::splat(2.0) * t123 * t729 * t117 - f64x8::splat(2.0) * t123 * t333 * t245 + f64x8::splat(2.0) * t759 * t256 + f64x8::splat(8.0) * t257 * t335 - f64x8::splat(6.0) * t756 * t566 + f64x8::splat(4.0) * t763 * t575;
            let t769 = -t248 * t733 + f64x8::splat(2.0) * t257 * t735 - f64x8::splat(2.0) * t739 * t738 + t125 * t729 / f64x8::splat(4.0) + t260 * t333 / f64x8::splat(4.0) + t344 * t245 / f64x8::splat(4.0) + t766 * t117 / f64x8::splat(4.0);
            let t773 = ((t59).select(t672 + t727, -f64x8::splat(8.0) / f64x8::splat(3.0) * t769 * t117 - f64x8::splat(8.0) / f64x8::splat(3.0) * t128 * t729 - f64x8::splat(8.0) / f64x8::splat(3.0) * t347 * t245 - f64x8::splat(8.0) / f64x8::splat(3.0) * t263 * t333));
            let t779 = t351 * t274 * t272;
            let t788 = f64x8::splat(1.0) / t611;
            let t790 = t132 * t788 * t272;
            let t791 = t33 * t448;
            let t793 = v_sigma * t791 * t598;
            let t797 = ((t2).select(f64x8::splat(0.0), -t46 * t351 * t138 * t18 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t46 * t773 * t19 * t18 + f64x8::splat(0.0040369036088841095) * t279 * t779 + f64x8::splat(0.003532290657773596) * t361 * t276 - f64x8::splat(0.0015138388533315413) * t361 * t267 * t357 * t272 - f64x8::splat(0.00014769346429286268) * t793 * t790));
            let tv2rhosigma0 = f64x8::splat(2.0) * t797 * v_rho + f64x8::splat(2.0) * t365;
            acc_v2rhosigma = tv2rhosigma0;
            let t800 = t378 * v_rho;
            let t802 = f64x8::splat(1.0) / t19 / t800;
            let t803 = t802 * t448;
            let t804 = t34 * t803;
            let t814 = f64x8::splat(0.0007762690503688975) * t804 * t446 * t439 - f64x8::splat(0.00010981960580929913) * t804 * t469 * t151 - f64x8::splat(0.00016988998695777817) * t804 * t476 * t151;
            let t815 = ((t60).select(t814, f64x8::splat(0.0)));
            let t816 = t815 * t234;
            let t818 = t815 * t238;
            let t822 = t815 * t226;
            let t824 = t815 * t230;
            let t826 = t815 * t214;
            let t828 = t815 * t218;
            let t830 = t815 * t222;
            let t832 = t815 * t206;
            let t834 = t815 * t210;
            let t836 = t815 * t194;
            let t838 = t815 * t198;
            let t840 = t815 * t202;
            let t842 = t815 * t186;
            let t844 = t815 * t190;
            let t846 = t815 * t144;
            let t848 = t815 * t178;
            let t850 = t815 * t182;
            let t852 = -f64x8::splat(34.0) / f64x8::splat(5985.0) * t816 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t818 - f64x8::splat(2.0) / f64x8::splat(9.0) * t815 * t241 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t822 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t824 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t826 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t828 + f64x8::splat(7.0) / f64x8::splat(870.0) * t830 + f64x8::splat(10.0) / f64x8::splat(693.0) * t832 - f64x8::splat(11.0) / f64x8::splat(897.0) * t834 - f64x8::splat(7.0) / f64x8::splat(270.0) * t836 + f64x8::splat(16.0) / f64x8::splat(765.0) * t838 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t840 - f64x8::splat(10.0) / f64x8::splat(231.0) * t842 + f64x8::splat(3.0) / f64x8::splat(91.0) * t844 + f64x8::splat(2.0) / f64x8::splat(15.0) * t846 - f64x8::splat(3.0) / f64x8::splat(35.0) * t848 + f64x8::splat(8.0) / f64x8::splat(135.0) * t850;
            let t853 = t295 * t295;
            let t854 = t853 * t391;
            let t858 = t853 * t94;
            let t860 = t853 * t97;
            let t862 = t853 * t100;
            let t864 = t853 * t103;
            let t866 = t853 * t106;
            let t868 = t853 * t109;
            let t870 = t853 * t112;
            let t872 = t853 * t70;
            let t874 = t853 * t73;
            let t876 = t853 * t76;
            let t878 = t853 * t79;
            let t880 = t853 * t82;
            let t882 = t853 * t85;
            let t884 = t853 * t88;
            let t886 = t853 * t91;
            let t888 = t853 * t67;
            let t890 = -f64x8::splat(18.0) / f64x8::splat(95.0) * t854 + f64x8::splat(2.0) / f64x8::splat(3.0) * t853 * t64 + f64x8::splat(11.0) / f64x8::splat(39.0) * t858 - f64x8::splat(24.0) / f64x8::splat(91.0) * t860 + f64x8::splat(26.0) / f64x8::splat(105.0) * t862 - f64x8::splat(7.0) / f64x8::splat(30.0) * t864 + f64x8::splat(15.0) / f64x8::splat(68.0) * t866 - f64x8::splat(32.0) / f64x8::splat(153.0) * t868 + f64x8::splat(34.0) / f64x8::splat(171.0) * t870 + f64x8::splat(3.0) / f64x8::splat(5.0) * t872 - f64x8::splat(8.0) / f64x8::splat(15.0) * t874 + f64x8::splat(10.0) / f64x8::splat(21.0) * t876 - f64x8::splat(3.0) / f64x8::splat(7.0) * t878 + f64x8::splat(7.0) / f64x8::splat(18.0) * t880 - f64x8::splat(16.0) / f64x8::splat(45.0) * t882 + f64x8::splat(18.0) / f64x8::splat(55.0) * t884 - f64x8::splat(10.0) / f64x8::splat(33.0) * t886 - f64x8::splat(2.0) / f64x8::splat(3.0) * t888;
            let t892 = ((t60).select(f64x8::splat(0.0), t814));
            let t897 = t121 * t892;
            let t899 = t333 * t333;
            let t900 = t255 * t899;
            let t903 = t547 * t899;
            let t918 = t248 * t899;
            let t921 = t248 * t892;
            let t924 = t550 * t899;
            let t927 = -f64x8::splat(2.0) * t123 * t892 * t117 + f64x8::splat(8.0) * t248 * t899 * t121 - f64x8::splat(2.0) * t123 * t899 + f64x8::splat(2.0) * t921 * t256 - f64x8::splat(6.0) * t918 * t566 + f64x8::splat(4.0) * t924 * t575;
            let t930 = -t248 * t897 + f64x8::splat(2.0) * t248 * t900 - f64x8::splat(2.0) * t550 * t903 + t125 * t892 / f64x8::splat(4.0) + t344 * t333 / f64x8::splat(2.0) + t927 * t117 / f64x8::splat(4.0);
            let t934 = ((t59).select(t852 + t890, -f64x8::splat(8.0) / f64x8::splat(3.0) * t128 * t892 - f64x8::splat(16.0) / f64x8::splat(3.0) * t347 * t333 - f64x8::splat(8.0) / f64x8::splat(3.0) * t930 * t117));
            let t943 = f64x8::splat(1.0) / t800;
            let t946 = t791 * t598;
            let t950 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t46 * t934 * t19 * t18 - f64x8::splat(0.0030276777066630825) * t361 * t351 * t357 * t272 + f64x8::splat(5.538504910982351e-05) * t946 * t132 * t943 * t272));
            let tv2sigma20 = f64x8::splat(2.0) * t950 * v_rho;
            acc_v2sigma2 = tv2sigma20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
