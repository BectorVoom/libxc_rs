//! GGA_X_ITYH fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ityh.c`
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
pub fn gga_x_ityh_fxc_unpol(
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
            let t18 = t3 / t4 * t17;
            let t19 = (simd::cbrt(v_rho));
            let t20 = t3 * t3;
            let t22 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = f64x8::splat(M_CBRT4);
            let t26 = t24 * t25;
            let t27 = t20 * t24;
            let t28 = t27 * t25;
            let t29 = f64x8::splat(M_CBRT2);
            let t30 = t29 * t29;
            let t31 = v_sigma * t30;
            let t32 = v_rho * v_rho;
            let t33 = t19 * t19;
            let t35 = f64x8::splat(1.0) / t33 / t32;
            let t36 = ((v_sigma).sqrt());
            let t37 = t36 * t29;
            let t39 = f64x8::splat(1.0) / t19 / v_rho;
            let t41 = (simd::ln(t37 * t39 + ((((t37 * t39) * (t37 * t39)) + f64x8::splat(1.0)).sqrt())));
            let t42 = t39 * t41;
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0252) * t37 * t42;
            let t46 = f64x8::splat(1.0) / t45;
            let t51 = f64x8::splat(1.0) + f64x8::splat(0.0009333333333333333) * t28 * t31 * t35 * t46;
            let t54 = f64x8::splat(M_PI) * t20 * t26 / t51;
            let t55 = ((t54).sqrt());
            let t57 = param_hyb_omega_0 / t55;
            let t58 = t11 * v_rho;
            let t59 = (simd::cbrt(t58));
            let t60 = f64x8::splat(1.0) / t59;
            let t61 = t29 * t60;
            let t63 = t57 * t61 / f64x8::splat(2.0);
            let t64 = (f64x8::splat(1.35)).simd_le(t63);
            let t65 = (f64x8::splat(1.35)).simd_lt(t63);
            let t66 = ((t65).select(t63, f64x8::splat(1.35)));
            let t67 = t66 * t66;
            let t70 = t67 * t67;
            let t71 = f64x8::splat(1.0) / t70;
            let t73 = t70 * t67;
            let t74 = f64x8::splat(1.0) / t73;
            let t76 = t70 * t70;
            let t77 = f64x8::splat(1.0) / t76;
            let t80 = f64x8::splat(1.0) / t76 / t67;
            let t83 = f64x8::splat(1.0) / t76 / t70;
            let t86 = f64x8::splat(1.0) / t76 / t73;
            let t88 = t76 * t76;
            let t89 = f64x8::splat(1.0) / t88;
            let t92 = ((t65).select(f64x8::splat(1.35), t63));
            let t93 = ((f64x8::splat(M_PI)).sqrt());
            let t94 = f64x8::splat(1.0) / t92;
            let t96 = (simd::erf(t94 / f64x8::splat(2.0)));
            let t98 = t92 * t92;
            let t99 = f64x8::splat(1.0) / t98;
            let t101 = (simd::exp(-t99 / f64x8::splat(4.0)));
            let t102 = t101 - f64x8::splat(1.0);
            let t105 = t101 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t98 * t102;
            let t108 = f64x8::splat(2.0) * t92 * t105 + t93 * t96;
            let t112 = ((t64).select(f64x8::splat(1.0) / t67 / f64x8::splat(36.0) - t71 / f64x8::splat(960.0) + t74 / f64x8::splat(26880.0) - t77 / f64x8::splat(829440.0) + t80 / f64x8::splat(28385280.0) - t83 / f64x8::splat(1073479680.0) + t86 / f64x8::splat(44590694400.0) - t89 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t92 * t108));
            let t113 = t19 * t112;
            let t117 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t113 * t51));
            let tzk0 = f64x8::splat(2.0) * t117;
            acc_zk = tzk0;
            let t118 = f64x8::splat(1.0) / t33;
            let t119 = t118 * t112;
            let t123 = t67 * t66;
            let t124 = f64x8::splat(1.0) / t123;
            let t127 = param_hyb_omega_0 / t55 / t54;
            let t129 = t127 * t61 * f64x8::splat(M_PI);
            let t130 = t51 * t51;
            let t131 = f64x8::splat(1.0) / t130;
            let t132 = t25 * t131;
            let t133 = t32 * v_rho;
            let t135 = f64x8::splat(1.0) / t33 / t133;
            let t140 = t25 * v_sigma;
            let t141 = t27 * t140;
            let t142 = t30 * t35;
            let t143 = t45 * t45;
            let t144 = f64x8::splat(1.0) / t143;
            let t147 = f64x8::splat(1.0) / t19 / t32 * t41;
            let t151 = t31 * t35 + f64x8::splat(1.0);
            let t152 = ((t151).sqrt());
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t135 * t153;
            let t157 = -f64x8::splat(0.0336) * t37 * t147 - f64x8::splat(0.0336) * t31 * t154;
            let t158 = t144 * t157;
            let t159 = t142 * t158;
            let t162 = -f64x8::splat(0.002488888888888889) * t28 * t31 * t135 * t46 - f64x8::splat(0.0009333333333333333) * t141 * t159;
            let t168 = f64x8::splat(1.0) / t59 / t58;
            let t169 = t29 * t168;
            let t173 = t129 * t27 * t132 * t162 / f64x8::splat(4.0) - t57 * t169 * t11 / f64x8::splat(6.0);
            let t174 = ((t65).select(t173, f64x8::splat(0.0)));
            let t177 = t70 * t66;
            let t178 = f64x8::splat(1.0) / t177;
            let t181 = t70 * t123;
            let t182 = f64x8::splat(1.0) / t181;
            let t186 = f64x8::splat(1.0) / t76 / t66;
            let t190 = f64x8::splat(1.0) / t76 / t123;
            let t194 = f64x8::splat(1.0) / t76 / t177;
            let t198 = f64x8::splat(1.0) / t76 / t181;
            let t202 = f64x8::splat(1.0) / t88 / t66;
            let t206 = ((t65).select(f64x8::splat(0.0), t173));
            let t208 = t101 * t99;
            let t212 = t98 * t92;
            let t213 = f64x8::splat(1.0) / t212;
            let t217 = t92 * t102;
            let t222 = t213 * t206 * t101 / f64x8::splat(2.0) - f64x8::splat(4.0) * t217 * t206 - t94 * t206 * t101;
            let t225 = f64x8::splat(2.0) * t206 * t105 - t208 * t206 + f64x8::splat(2.0) * t92 * t222;
            let t229 = ((t64).select(-t124 * t174 / f64x8::splat(18.0) + t178 * t174 / f64x8::splat(240.0) - t182 * t174 / f64x8::splat(4480.0) + t186 * t174 / f64x8::splat(103680.0) - t190 * t174 / f64x8::splat(2838528.0) + t194 * t174 / f64x8::splat(89456640.0) - t198 * t174 / f64x8::splat(3185049600.0) + t202 * t174 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t206 * t108 - f64x8::splat(8.0) / f64x8::splat(3.0) * t92 * t225));
            let t230 = t19 * t229;
            let t238 = ((t2).select(f64x8::splat(0.0), -t18 * t119 * t51 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t230 * t51 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t113 * t162));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t238 + f64x8::splat(2.0) * t117;
            acc_vrho = tvrho0;
            let t245 = f64x8::splat(1.0) / t36 * t29;
            let t250 = f64x8::splat(0.0126) * t245 * t42 + f64x8::splat(0.0126) * t142 * t153;
            let t251 = t144 * t250;
            let t252 = t142 * t251;
            let t255 = f64x8::splat(0.0009333333333333333) * t28 * t142 * t46 - f64x8::splat(0.0009333333333333333) * t141 * t252;
            let t259 = t129 * t27 * t132 * t255 / f64x8::splat(4.0);
            let t260 = ((t65).select(t259, f64x8::splat(0.0)));
            let t263 = t178 * t260;
            let t265 = t182 * t260;
            let t267 = t186 * t260;
            let t269 = t190 * t260;
            let t271 = t194 * t260;
            let t273 = t198 * t260;
            let t275 = t202 * t260;
            let t278 = ((t65).select(f64x8::splat(0.0), t259));
            let t290 = t213 * t278 * t101 / f64x8::splat(2.0) - f64x8::splat(4.0) * t217 * t278 - t94 * t278 * t101;
            let t293 = f64x8::splat(2.0) * t278 * t105 - t208 * t278 + f64x8::splat(2.0) * t92 * t290;
            let t297 = ((t64).select(-t124 * t260 / f64x8::splat(18.0) + t263 / f64x8::splat(240.0) - t265 / f64x8::splat(4480.0) + t267 / f64x8::splat(103680.0) - t269 / f64x8::splat(2838528.0) + t271 / f64x8::splat(89456640.0) - t273 / f64x8::splat(3185049600.0) + t275 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t278 * t108 - f64x8::splat(8.0) / f64x8::splat(3.0) * t92 * t293));
            let t298 = t19 * t297;
            let t305 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t113 * t255 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t298 * t51));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t305;
            acc_vsigma = tvsigma0;
            let t309 = f64x8::splat(1.0) / t33 / v_rho;
            let t310 = t309 * t112;
            let t314 = t118 * t229;
            let t321 = t174 * t174;
            let t324 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t326 = t23 * t23;
            let t327 = f64x8::splat(1.0) / t326;
            let t328 = t25 * t25;
            let t329 = t327 * t328;
            let t335 = param_hyb_omega_0 / t55 / t324 / t3 / t329 / t131 / f64x8::splat(3.0);
            let t337 = t335 * t61 * t324;
            let t338 = t3 * t327;
            let t339 = t130 * t130;
            let t340 = f64x8::splat(1.0) / t339;
            let t341 = t328 * t340;
            let t342 = t162 * t162;
            let t348 = t127 * t169 * f64x8::splat(M_PI);
            let t349 = t131 * t162;
            let t354 = t130 * t51;
            let t355 = f64x8::splat(1.0) / t354;
            let t356 = t25 * t355;
            let t361 = t32 * t32;
            let t363 = f64x8::splat(1.0) / t33 / t361;
            let t368 = t30 * t135;
            let t369 = t368 * t158;
            let t373 = f64x8::splat(1.0) / t143 / t45;
            let t374 = t157 * t157;
            let t375 = t373 * t374;
            let t376 = t142 * t375;
            let t381 = f64x8::splat(1.0) / t19 / t133 * t41;
            let t384 = t363 * t153;
            let t387 = v_sigma * v_sigma;
            let t388 = t387 * t29;
            let t391 = f64x8::splat(1.0) / t19 / t361 / t133;
            let t393 = f64x8::splat(1.0) / t152 / t151;
            let t397 = f64x8::splat(0.0784) * t37 * t381 + f64x8::splat(0.168) * t31 * t384 - f64x8::splat(0.0896) * t388 * t391 * t393;
            let t398 = t144 * t397;
            let t399 = t142 * t398;
            let t402 = f64x8::splat(0.009125925925925926) * t28 * t31 * t363 * t46 + f64x8::splat(0.004977777777777778) * t141 * t369 + f64x8::splat(0.0018666666666666666) * t141 * t376 - f64x8::splat(0.0009333333333333333) * t141 * t399;
            let t407 = t11 * t11;
            let t410 = f64x8::splat(1.0) / t59 / t407 / t32;
            let t411 = t29 * t410;
            let t415 = f64x8::splat(9.0) / f64x8::splat(8.0) * t337 * t338 * t341 * t342 - t348 * t28 * t349 * t11 / f64x8::splat(6.0) - t129 * t27 * t356 * t342 / f64x8::splat(2.0) + t129 * t27 * t132 * t402 / f64x8::splat(4.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t57 * t411 * t407;
            let t416 = ((t65).select(t415, f64x8::splat(0.0)));
            let t444 = f64x8::splat(1.0) / t88 / t67;
            let t449 = t71 * t321 / f64x8::splat(6.0) - t124 * t416 / f64x8::splat(18.0) - t74 * t321 / f64x8::splat(48.0) + t178 * t416 / f64x8::splat(240.0) + t77 * t321 / f64x8::splat(640.0) - t182 * t416 / f64x8::splat(4480.0) - t80 * t321 / f64x8::splat(11520.0) + t186 * t416 / f64x8::splat(103680.0) + t83 * t321 / f64x8::splat(258048.0) - t190 * t416 / f64x8::splat(2838528.0) - t86 * t321 / f64x8::splat(6881280.0) + t194 * t416 / f64x8::splat(89456640.0) + t89 * t321 / f64x8::splat(212336640.0) - t198 * t416 / f64x8::splat(3185049600.0) - t444 * t321 / f64x8::splat(7431782400.0) + t202 * t416 / f64x8::splat(126340300800.0);
            let t450 = ((t65).select(f64x8::splat(0.0), t415));
            let t455 = t98 * t98;
            let t457 = f64x8::splat(1.0) / t455 / t92;
            let t458 = t206 * t206;
            let t459 = t457 * t458;
            let t462 = t101 * t213;
            let t470 = f64x8::splat(1.0) / t455;
            let t478 = f64x8::splat(1.0) / t455 / t98;
            let t479 = t478 * t458;
            let t490 = -f64x8::splat(2.0) * t470 * t458 * t101 + t213 * t450 * t101 / f64x8::splat(2.0) + t479 * t101 / f64x8::splat(4.0) - f64x8::splat(4.0) * t458 * t102 - t99 * t458 * t101 - f64x8::splat(4.0) * t217 * t450 - t94 * t450 * t101;
            let t493 = -t459 * t101 / f64x8::splat(2.0) + f64x8::splat(2.0) * t462 * t458 - t208 * t450 + f64x8::splat(2.0) * t450 * t105 + f64x8::splat(4.0) * t206 * t222 + f64x8::splat(2.0) * t92 * t490;
            let t497 = ((t64).select(t449, -f64x8::splat(8.0) / f64x8::splat(3.0) * t450 * t108 - f64x8::splat(16.0) / f64x8::splat(3.0) * t206 * t225 - f64x8::splat(8.0) / f64x8::splat(3.0) * t92 * t493));
            let t498 = t19 * t497;
            let t509 = ((t2).select(f64x8::splat(0.0), t18 * t310 * t51 / f64x8::splat(12.0) - t18 * t314 * t51 / f64x8::splat(4.0) - t18 * t119 * t162 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t498 * t51 - f64x8::splat(3.0) / f64x8::splat(4.0) * t18 * t230 * t162 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t113 * t402));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t509 + f64x8::splat(4.0) * t238;
            acc_v2rho2 = tv2rho20;
            let t512 = t118 * t297;
            let t516 = t71 * t260;
            let t519 = t338 * t328;
            let t520 = t340 * t255;
            let t525 = t131 * t255;
            let t530 = t355 * t255;
            let t540 = t368 * t251;
            let t543 = t373 * t250;
            let t544 = t543 * t157;
            let t552 = t361 * t32;
            let t554 = f64x8::splat(1.0) / t19 / t552;
            let t555 = t29 * t554;
            let t556 = t393 * v_sigma;
            let t559 = -f64x8::splat(0.0168) * t245 * t147 - f64x8::splat(0.0504) * t368 * t153 + f64x8::splat(0.0336) * t555 * t556;
            let t560 = t144 * t559;
            let t561 = t142 * t560;
            let t564 = -f64x8::splat(0.002488888888888889) * t28 * t368 * t46 - f64x8::splat(0.0009333333333333333) * t28 * t159 + f64x8::splat(0.002488888888888889) * t141 * t540 + f64x8::splat(0.0018666666666666666) * t141 * t142 * t544 - f64x8::splat(0.0009333333333333333) * t141 * t561;
            let t569 = f64x8::splat(9.0) / f64x8::splat(8.0) * t337 * t519 * t520 * t162 - t348 * t28 * t525 * t11 / f64x8::splat(12.0) - t129 * t28 * t530 * t162 / f64x8::splat(2.0) + t129 * t27 * t132 * t564 / f64x8::splat(4.0);
            let t570 = ((t65).select(t569, f64x8::splat(0.0)));
            let t573 = t74 * t260;
            let t576 = t178 * t570;
            let t578 = t77 * t260;
            let t581 = t182 * t570;
            let t583 = t80 * t260;
            let t586 = t186 * t570;
            let t588 = t83 * t260;
            let t591 = t190 * t570;
            let t593 = t86 * t260;
            let t596 = t194 * t570;
            let t598 = t89 * t260;
            let t601 = t198 * t570;
            let t603 = t444 * t260;
            let t606 = t202 * t570;
            let t608 = t516 * t174 / f64x8::splat(6.0) - t124 * t570 / f64x8::splat(18.0) - t573 * t174 / f64x8::splat(48.0) + t576 / f64x8::splat(240.0) + t578 * t174 / f64x8::splat(640.0) - t581 / f64x8::splat(4480.0) - t583 * t174 / f64x8::splat(11520.0) + t586 / f64x8::splat(103680.0) + t588 * t174 / f64x8::splat(258048.0) - t591 / f64x8::splat(2838528.0) - t593 * t174 / f64x8::splat(6881280.0) + t596 / f64x8::splat(89456640.0) + t598 * t174 / f64x8::splat(212336640.0) - t601 / f64x8::splat(3185049600.0) - t603 * t174 / f64x8::splat(7431782400.0) + t606 / f64x8::splat(126340300800.0);
            let t609 = ((t65).select(f64x8::splat(0.0), t569));
            let t613 = t457 * t206;
            let t614 = t101 * t278;
            let t617 = t278 * t206;
            let t627 = t470 * t278;
            let t628 = t101 * t206;
            let t634 = t478 * t278;
            let t637 = t206 * t102;
            let t640 = t99 * t206;
            let t646 = -f64x8::splat(2.0) * t627 * t628 + t213 * t609 * t101 / f64x8::splat(2.0) + t634 * t628 / f64x8::splat(4.0) - f64x8::splat(4.0) * t637 * t278 - t640 * t614 - f64x8::splat(4.0) * t217 * t609 - t94 * t609 * t101;
            let t649 = -t613 * t614 / f64x8::splat(2.0) + f64x8::splat(2.0) * t462 * t617 - t208 * t609 + f64x8::splat(2.0) * t609 * t105 + f64x8::splat(2.0) * t278 * t222 + f64x8::splat(2.0) * t206 * t290 + f64x8::splat(2.0) * t92 * t646;
            let t653 = ((t64).select(t608, -f64x8::splat(8.0) / f64x8::splat(3.0) * t609 * t108 - f64x8::splat(8.0) / f64x8::splat(3.0) * t206 * t293 - f64x8::splat(8.0) / f64x8::splat(3.0) * t278 * t225 - f64x8::splat(8.0) / f64x8::splat(3.0) * t92 * t649));
            let t654 = t19 * t653;
            let t671 = ((t2).select(f64x8::splat(0.0), -t18 * t512 * t51 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t654 * t51 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t298 * t162 - t18 * t119 * t255 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t230 * t255 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t113 * t564));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t671 + f64x8::splat(2.0) * t305;
            acc_v2rhosigma = tv2rhosigma0;
            let t674 = t260 * t260;
            let t677 = t255 * t255;
            let t688 = t250 * t250;
            let t689 = t373 * t688;
            let t690 = t142 * t689;
            let t695 = f64x8::splat(1.0) / t36 / v_sigma * t29;
            let t698 = f64x8::splat(1.0) / v_sigma;
            let t699 = t698 * t30;
            let t700 = t35 * t153;
            let t703 = t361 * v_rho;
            let t705 = f64x8::splat(1.0) / t19 / t703;
            let t709 = -f64x8::splat(0.0063) * t695 * t42 + f64x8::splat(0.0063) * t699 * t700 - f64x8::splat(0.0126) * t29 * t705 * t393;
            let t710 = t144 * t709;
            let t711 = t142 * t710;
            let t714 = -f64x8::splat(0.0018666666666666666) * t28 * t252 + f64x8::splat(0.0018666666666666666) * t141 * t690 - f64x8::splat(0.0009333333333333333) * t141 * t711;
            let t719 = f64x8::splat(9.0) / f64x8::splat(8.0) * t337 * t338 * t341 * t677 - t129 * t27 * t356 * t677 / f64x8::splat(2.0) + t129 * t27 * t132 * t714 / f64x8::splat(4.0);
            let t720 = ((t65).select(t719, f64x8::splat(0.0)));
            let t723 = t74 * t674;
            let t725 = t178 * t720;
            let t727 = t77 * t674;
            let t729 = t182 * t720;
            let t731 = t80 * t674;
            let t733 = t186 * t720;
            let t735 = t83 * t674;
            let t737 = t190 * t720;
            let t739 = t86 * t674;
            let t741 = t194 * t720;
            let t743 = t89 * t674;
            let t745 = t198 * t720;
            let t747 = t444 * t674;
            let t749 = t202 * t720;
            let t751 = t71 * t674 / f64x8::splat(6.0) - t124 * t720 / f64x8::splat(18.0) - t723 / f64x8::splat(48.0) + t725 / f64x8::splat(240.0) + t727 / f64x8::splat(640.0) - t729 / f64x8::splat(4480.0) - t731 / f64x8::splat(11520.0) + t733 / f64x8::splat(103680.0) + t735 / f64x8::splat(258048.0) - t737 / f64x8::splat(2838528.0) - t739 / f64x8::splat(6881280.0) + t741 / f64x8::splat(89456640.0) + t743 / f64x8::splat(212336640.0) - t745 / f64x8::splat(3185049600.0) - t747 / f64x8::splat(7431782400.0) + t749 / f64x8::splat(126340300800.0);
            let t752 = ((t65).select(f64x8::splat(0.0), t719));
            let t757 = t278 * t278;
            let t758 = t457 * t757;
            let t774 = t478 * t757;
            let t785 = -f64x8::splat(2.0) * t470 * t757 * t101 + t213 * t752 * t101 / f64x8::splat(2.0) + t774 * t101 / f64x8::splat(4.0) - f64x8::splat(4.0) * t757 * t102 - t99 * t757 * t101 - f64x8::splat(4.0) * t217 * t752 - t94 * t752 * t101;
            let t788 = -t758 * t101 / f64x8::splat(2.0) + f64x8::splat(2.0) * t462 * t757 - t208 * t752 + f64x8::splat(2.0) * t752 * t105 + f64x8::splat(4.0) * t278 * t290 + f64x8::splat(2.0) * t92 * t785;
            let t792 = ((t64).select(t751, -f64x8::splat(8.0) / f64x8::splat(3.0) * t752 * t108 - f64x8::splat(16.0) / f64x8::splat(3.0) * t278 * t293 - f64x8::splat(8.0) / f64x8::splat(3.0) * t92 * t788));
            let t793 = t19 * t792;
            let t804 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t793 * t51 - f64x8::splat(3.0) / f64x8::splat(4.0) * t18 * t298 * t255 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t113 * t714));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t804;
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
