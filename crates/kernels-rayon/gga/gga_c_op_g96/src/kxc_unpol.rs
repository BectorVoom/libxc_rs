//! GGA_C_OP_G96 kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_g96.c`
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
pub fn gga_c_op_g96_kxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v3rho2sigma = V_ZERO;
        let mut acc_v3rhosigma2 = V_ZERO;
        let mut acc_v3sigma3 = V_ZERO;
        {
            let t1 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t4 = (t1) | ((v_rho / f64x8::splat(2.0)).simd_le(dens_threshold));
            let t5 = zeta_threshold - f64x8::splat(1.0);
            let t6 = -t5;
            let t7 = ((t1).select(t5, (t1).select(t6, f64x8::splat(0.0))));
            let t8 = t7 * t7;
            let t9 = f64x8::splat(1.0) - t8;
            let t10 = t9 * v_rho;
            let t11 = f64x8::splat(1.0) + t7;
            let t14 = (t11 * v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t15 = f64x8::splat(M_CBRT3);
            let t16 = t15 * t15;
            let t18 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t19 = f64x8::splat(1.0) / t18;
            let t20 = t16 * t19;
            let t21 = f64x8::splat(M_CBRT4);
            let t22 = t20 * t21;
            let t23 = f64x8::splat(M_CBRT2);
            let t24 = (t11).simd_le(zeta_threshold);
            let t25 = f64x8::splat(1.0) - t7;
            let t26 = (t25).simd_le(zeta_threshold);
            let t27 = ((t24).select(t5, (t26).select(t6, t7)));
            let t28 = f64x8::splat(1.0) + t27;
            let t29 = t28 * v_rho;
            let t30 = (simd::cbrt(t29));
            let t31 = f64x8::splat(1.0) / t30;
            let t33 = ((v_sigma).sqrt());
            let t34 = t33 * t23;
            let t35 = (simd::cbrt(v_rho));
            let t37 = f64x8::splat(1.0) / t35 / v_rho;
            let t38 = t34 * t37;
            let t39 = ((t38).sqrt());
            let t40 = t39 * t38;
            let t44 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(1233.0) * t20 * t21 * t40;
            let t45 = f64x8::splat(1.0) / t44;
            let t49 = ((t14).select(f64x8::splat(0.0), t22 * t23 * t31 * t45 / f64x8::splat(9.0)));
            let t53 = (t25 * v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t54 = ((t26).select(t5, (t24).select(t6, -t7)));
            let t55 = f64x8::splat(1.0) + t54;
            let t56 = t55 * v_rho;
            let t57 = (simd::cbrt(t56));
            let t58 = f64x8::splat(1.0) / t57;
            let t63 = ((t53).select(f64x8::splat(0.0), t22 * t23 * t58 * t45 / f64x8::splat(9.0)));
            let t64 = t49 + t63;
            let t65 = (t64).simd_eq(f64x8::splat(0.0));
            let t66 = ((t65).select(f64x8::splat(f64::EPSILON), t64));
            let t69 = f64x8::splat(3.59628532) / t66 + f64x8::splat(0.5764);
            let t70 = t66 * t66;
            let t71 = t70 * t70;
            let t72 = f64x8::splat(1.0) / t71;
            let t74 = t70 * t66;
            let t75 = f64x8::splat(1.0) / t74;
            let t77 = f64x8::splat(1.0) / t70;
            let t79 = f64x8::splat(31.220719919544194) * t72 + f64x8::splat(14.903739892213245) * t75 + f64x8::splat(1.778517305052) * t77;
            let t80 = f64x8::splat(1.0) / t79;
            let tzk0 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t10 * t69 * t80));
            acc_zk = tzk0;
            let t84 = t9 * t69;
            let t88 = f64x8::splat(1.0) / t30 / t29;
            let t94 = t18 * t18;
            let t95 = f64x8::splat(1.0) / t94;
            let t96 = t15 * t95;
            let t97 = t21 * t21;
            let t98 = t23 * t23;
            let t99 = t97 * t98;
            let t100 = t96 * t99;
            let t101 = t44 * t44;
            let t102 = f64x8::splat(1.0) / t101;
            let t103 = t31 * t102;
            let t104 = t39 * t33;
            let t105 = v_rho * v_rho;
            let t107 = f64x8::splat(1.0) / t35 / t105;
            let t108 = t104 * t107;
            let t113 = ((t14).select(f64x8::splat(0.0), -t22 * t23 * t88 * t45 * t28 / f64x8::splat(27.0) + f64x8::splat(4.0) / f64x8::splat(3699.0) * t100 * t103 * t108));
            let t115 = f64x8::splat(1.0) / t57 / t56;
            let t121 = t58 * t102;
            let t126 = ((t53).select(f64x8::splat(0.0), -t22 * t23 * t115 * t45 * t55 / f64x8::splat(27.0) + f64x8::splat(4.0) / f64x8::splat(3699.0) * t100 * t121 * t108));
            let t128 = ((t65).select(f64x8::splat(0.0), t113 + t126));
            let t133 = t79 * t79;
            let t134 = f64x8::splat(1.0) / t133;
            let t135 = t69 * t134;
            let t137 = f64x8::splat(1.0) / t71 / t66;
            let t138 = t137 * t128;
            let t140 = t72 * t128;
            let t144 = -f64x8::splat(124.88287967817678) * t138 - f64x8::splat(44.711219676639736) * t140 - f64x8::splat(3.557034610104) * t75 * t128;
            let t149 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t84 * t80 + f64x8::splat(0.89907133) * t10 * t77 * t128 * t80 + f64x8::splat(0.25) * t10 * t135 * t144));
            let tvrho0 = v_rho * t149 + tzk0;
            acc_vrho = tvrho0;
            let t151 = f64x8::splat(1.0) / t33;
            let t152 = t39 * t151;
            let t153 = t152 * t37;
            let t157 = ((t14).select(f64x8::splat(0.0), -t100 * t103 * t153 / f64x8::splat(2466.0)));
            let t161 = ((t53).select(f64x8::splat(0.0), -t100 * t121 * t153 / f64x8::splat(2466.0)));
            let t163 = ((t65).select(f64x8::splat(0.0), t157 + t161));
            let t168 = t137 * t163;
            let t170 = t72 * t163;
            let t172 = t75 * t163;
            let t174 = -f64x8::splat(124.88287967817678) * t168 - f64x8::splat(44.711219676639736) * t170 - f64x8::splat(3.557034610104) * t172;
            let t179 = ((t4).select(f64x8::splat(0.0), f64x8::splat(0.89907133) * t10 * t77 * t163 * t80 + f64x8::splat(0.25) * t10 * t135 * t174));
            let tvsigma0 = v_rho * t179;
            acc_vsigma = tvsigma0;
            let t181 = t9 * t77;
            let t182 = t128 * t80;
            let t188 = t128 * t128;
            let t193 = t28 * t28;
            let t196 = f64x8::splat(1.0) / t30 / t193 / t105;
            let t203 = t96 * t99 * t88;
            let t204 = t102 * t28;
            let t208 = f64x8::splat(M_PI) * t31;
            let t210 = f64x8::splat(1.0) / t101 / t44;
            let t211 = t208 * t210;
            let t212 = t33 * v_sigma;
            let t213 = t212 * t23;
            let t214 = t105 * t105;
            let t215 = t214 * t105;
            let t216 = f64x8::splat(1.0) / t215;
            let t217 = t213 * t216;
            let t221 = t96 * t97 * t31;
            let t222 = f64x8::splat(1.0) / t39;
            let t223 = t102 * t222;
            let t224 = t35 * t35;
            let t226 = f64x8::splat(1.0) / t224 / t214;
            let t228 = t223 * v_sigma * t226;
            let t231 = t105 * v_rho;
            let t233 = f64x8::splat(1.0) / t35 / t231;
            let t234 = t104 * t233;
            let t239 = ((t14).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(81.0) * t22 * t23 * t196 * t45 * t193 - f64x8::splat(8.0) / f64x8::splat(11097.0) * t203 * t204 * t108 + f64x8::splat(256.0) / f64x8::splat(1520289.0) * t211 * t217 - f64x8::splat(16.0) / f64x8::splat(11097.0) * t221 * t228 - f64x8::splat(28.0) / f64x8::splat(11097.0) * t100 * t103 * t234));
            let t240 = t55 * t55;
            let t243 = f64x8::splat(1.0) / t57 / t240 / t105;
            let t250 = t96 * t99 * t115;
            let t251 = t102 * t55;
            let t255 = f64x8::splat(M_PI) * t58;
            let t256 = t255 * t210;
            let t260 = t96 * t97 * t58;
            let t267 = ((t53).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(81.0) * t22 * t23 * t243 * t45 * t240 - f64x8::splat(8.0) / f64x8::splat(11097.0) * t250 * t251 * t108 + f64x8::splat(256.0) / f64x8::splat(1520289.0) * t256 * t217 - f64x8::splat(16.0) / f64x8::splat(11097.0) * t260 * t228 - f64x8::splat(28.0) / f64x8::splat(11097.0) * t100 * t121 * t234));
            let t269 = ((t65).select(f64x8::splat(0.0), t239 + t267));
            let t274 = t10 * t77;
            let t275 = t128 * t134;
            let t276 = t275 * t144;
            let t280 = f64x8::splat(1.0) / t133 / t79;
            let t281 = t69 * t280;
            let t282 = t144 * t144;
            let t287 = f64x8::splat(1.0) / t71 / t70;
            let t288 = t287 * t188;
            let t292 = t137 * t188;
            let t300 = f64x8::splat(624.4143983908839) * t288 - f64x8::splat(124.88287967817678) * t137 * t269 + f64x8::splat(178.84487870655894) * t292 - f64x8::splat(44.711219676639736) * t72 * t269 + f64x8::splat(10.671103830312) * t72 * t188 - f64x8::splat(3.557034610104) * t75 * t269;
            let t305 = ((t4).select(f64x8::splat(0.0), f64x8::splat(1.79814266) * t181 * t182 + f64x8::splat(0.5) * t84 * t134 * t144 - f64x8::splat(1.79814266) * t10 * t75 * t188 * t80 + f64x8::splat(0.89907133) * t10 * t77 * t269 * t80 - f64x8::splat(1.79814266) * t274 * t276 - f64x8::splat(0.5) * t10 * t281 * t282 + f64x8::splat(0.25) * t10 * t135 * t300));
            let tv2rho20 = v_rho * t305 + f64x8::splat(2.0) * t149;
            acc_v2rho2 = tv2rho20;
            let t307 = t163 * t80;
            let t310 = t10 * t75;
            let t311 = t307 * t128;
            let t314 = t102 * t39;
            let t315 = t151 * t37;
            let t320 = t214 * v_rho;
            let t321 = f64x8::splat(1.0) / t320;
            let t322 = t34 * t321;
            let t325 = t96 * t97;
            let t327 = f64x8::splat(1.0) / t224 / t231;
            let t328 = t222 * t327;
            let t332 = t152 * t107;
            let t337 = ((t14).select(f64x8::splat(0.0), t203 * t314 * t315 * t28 / f64x8::splat(7398.0) - f64x8::splat(32.0) / f64x8::splat(506763.0) * t211 * t322 + f64x8::splat(2.0) / f64x8::splat(3699.0) * t325 * t103 * t328 + f64x8::splat(2.0) / f64x8::splat(3699.0) * t100 * t103 * t332));
            let t351 = ((t53).select(f64x8::splat(0.0), t250 * t314 * t315 * t55 / f64x8::splat(7398.0) - f64x8::splat(32.0) / f64x8::splat(506763.0) * t256 * t322 + f64x8::splat(2.0) / f64x8::splat(3699.0) * t325 * t121 * t328 + f64x8::splat(2.0) / f64x8::splat(3699.0) * t100 * t121 * t332));
            let t353 = ((t65).select(f64x8::splat(0.0), t337 + t351));
            let t358 = t163 * t134;
            let t359 = t358 * t144;
            let t365 = t275 * t174;
            let t368 = t10 * t69;
            let t369 = t280 * t174;
            let t370 = t369 * t144;
            let t373 = t287 * t163;
            let t376 = t137 * t353;
            let t380 = t72 * t353;
            let t386 = f64x8::splat(624.4143983908839) * t373 * t128 - f64x8::splat(124.88287967817678) * t376 + f64x8::splat(178.84487870655894) * t168 * t128 - f64x8::splat(44.711219676639736) * t380 + f64x8::splat(10.671103830312) * t170 * t128 - f64x8::splat(3.557034610104) * t75 * t353;
            let t391 = ((t4).select(f64x8::splat(0.0), f64x8::splat(0.89907133) * t181 * t307 - f64x8::splat(1.79814266) * t310 * t311 + f64x8::splat(0.89907133) * t10 * t77 * t353 * t80 - f64x8::splat(0.89907133) * t274 * t359 + f64x8::splat(0.25) * t84 * t134 * t174 - f64x8::splat(0.89907133) * t274 * t365 - f64x8::splat(0.5) * t368 * t370 + f64x8::splat(0.25) * t10 * t135 * t386));
            let tv2rhosigma0 = v_rho * t391 + t179;
            acc_v2rhosigma = tv2rhosigma0;
            let t393 = t163 * t163;
            let t398 = t151 * t23;
            let t399 = f64x8::splat(1.0) / t214;
            let t400 = t398 * t399;
            let t403 = f64x8::splat(1.0) / v_sigma;
            let t405 = f64x8::splat(1.0) / t224 / t105;
            let t406 = t403 * t405;
            let t407 = t223 * t406;
            let t410 = f64x8::splat(1.0) / t212;
            let t411 = t39 * t410;
            let t412 = t411 * t37;
            let t417 = ((t14).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(168921.0) * t211 * t400 - t221 * t407 / f64x8::splat(4932.0) + t100 * t103 * t412 / f64x8::splat(4932.0)));
            let t426 = ((t53).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(168921.0) * t256 * t400 - t260 * t407 / f64x8::splat(4932.0) + t100 * t121 * t412 / f64x8::splat(4932.0)));
            let t428 = ((t65).select(f64x8::splat(0.0), t417 + t426));
            let t433 = t358 * t174;
            let t436 = t174 * t174;
            let t440 = t287 * t393;
            let t442 = t137 * t428;
            let t444 = t137 * t393;
            let t446 = t72 * t428;
            let t452 = f64x8::splat(624.4143983908839) * t440 - f64x8::splat(124.88287967817678) * t442 + f64x8::splat(178.84487870655894) * t444 - f64x8::splat(44.711219676639736) * t446 + f64x8::splat(10.671103830312) * t72 * t393 - f64x8::splat(3.557034610104) * t75 * t428;
            let t457 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(1.79814266) * t10 * t75 * t393 * t80 + f64x8::splat(0.89907133) * t10 * t77 * t428 * t80 - f64x8::splat(1.79814266) * t274 * t433 - f64x8::splat(0.5) * t10 * t281 * t436 + f64x8::splat(0.25) * t10 * t135 * t452));
            let tv2sigma20 = v_rho * t457;
            acc_v2sigma2 = tv2sigma20;
            let t459 = t188 * t128;
            let t464 = t182 * t269;
            let t467 = t188 * t134;
            let t468 = t467 * t144;
            let t471 = t269 * t134;
            let t472 = t471 * t144;
            let t475 = t275 * t300;
            let t478 = t133 * t133;
            let t479 = f64x8::splat(1.0) / t478;
            let t480 = t69 * t479;
            let t481 = t282 * t144;
            let t485 = t280 * t144;
            let t486 = t485 * t300;
            let t495 = t193 * t28;
            let t498 = f64x8::splat(1.0) / t30 / t495 / t231;
            let t505 = t96 * t99 * t196;
            let t506 = t102 * t193;
            let t510 = f64x8::splat(M_PI) * t88;
            let t511 = t510 * t210;
            let t512 = t28 * t212;
            let t513 = t23 * t216;
            let t517 = t97 * t88;
            let t518 = t96 * t517;
            let t519 = t222 * v_sigma;
            let t520 = t519 * t226;
            let t527 = t101 * t101;
            let t528 = f64x8::splat(1.0) / t527;
            let t529 = v_sigma * v_sigma;
            let t531 = t528 * t529 * t98;
            let t532 = t208 * t531;
            let t533 = t214 * t214;
            let t535 = f64x8::splat(1.0) / t35 / t533;
            let t537 = t19 * t21;
            let t538 = t537 * t39;
            let t539 = t535 * t16 * t538;
            let t542 = t214 * t231;
            let t543 = f64x8::splat(1.0) / t542;
            let t544 = t213 * t543;
            let t547 = f64x8::splat(1.0) / t40;
            let t548 = t102 * t547;
            let t549 = t548 * t544;
            let t553 = f64x8::splat(1.0) / t224 / t320;
            let t555 = t223 * v_sigma * t553;
            let t559 = f64x8::splat(1.0) / t35 / t214;
            let t560 = t104 * t559;
            let t565 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(28.0) / f64x8::splat(243.0) * t22 * t23 * t498 * t45 * t495 + f64x8::splat(16.0) / f64x8::splat(11097.0) * t505 * t506 * t108 - f64x8::splat(256.0) / f64x8::splat(1520289.0) * t511 * t512 * t513 + f64x8::splat(16.0) / f64x8::splat(11097.0) * t518 * t204 * t520 + f64x8::splat(28.0) / f64x8::splat(11097.0) * t203 * t204 * t234 + f64x8::splat(1024.0) / f64x8::splat(624838779.0) * t532 * t539 - f64x8::splat(256.0) / f64x8::splat(168921.0) * t211 * t544 - f64x8::splat(32.0) / f64x8::splat(33291.0) * t221 * t549 + f64x8::splat(112.0) / f64x8::splat(11097.0) * t221 * t555 + f64x8::splat(280.0) / f64x8::splat(33291.0) * t100 * t103 * t560));
            let t566 = t240 * t55;
            let t569 = f64x8::splat(1.0) / t57 / t566 / t231;
            let t576 = t96 * t99 * t243;
            let t577 = t102 * t240;
            let t581 = f64x8::splat(M_PI) * t115;
            let t582 = t581 * t210;
            let t583 = t55 * t212;
            let t587 = t97 * t115;
            let t588 = t96 * t587;
            let t595 = t255 * t531;
            let t608 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(28.0) / f64x8::splat(243.0) * t22 * t23 * t569 * t45 * t566 + f64x8::splat(16.0) / f64x8::splat(11097.0) * t576 * t577 * t108 - f64x8::splat(256.0) / f64x8::splat(1520289.0) * t582 * t583 * t513 + f64x8::splat(16.0) / f64x8::splat(11097.0) * t588 * t251 * t520 + f64x8::splat(28.0) / f64x8::splat(11097.0) * t250 * t251 * t234 + f64x8::splat(1024.0) / f64x8::splat(624838779.0) * t595 * t539 - f64x8::splat(256.0) / f64x8::splat(168921.0) * t256 * t544 - f64x8::splat(32.0) / f64x8::splat(33291.0) * t260 * t549 + f64x8::splat(112.0) / f64x8::splat(11097.0) * t260 * t555 + f64x8::splat(280.0) / f64x8::splat(33291.0) * t100 * t121 * t560));
            let t610 = ((t65).select(f64x8::splat(0.0), t565 + t608));
            let t616 = f64x8::splat(1.0) / t71 / t74;
            let t619 = t287 * t128;
            let t636 = -f64x8::splat(3746.4863903453033) * t616 * t459 + f64x8::splat(1873.2431951726517) * t619 * t269 - f64x8::splat(124.88287967817678) * t137 * t610 - f64x8::splat(894.2243935327947) * t287 * t459 + f64x8::splat(536.5346361196769) * t138 * t269 - f64x8::splat(44.711219676639736) * t72 * t610 - f64x8::splat(42.684415321248) * t137 * t459 + f64x8::splat(32.013311490936) * t140 * t269 - f64x8::splat(3.557034610104) * t75 * t610;
            let t640 = t9 * t75;
            let t641 = t188 * t80;
            let t649 = t128 * t280;
            let t650 = t649 * t282;
            let t653 = f64x8::splat(5.39442798) * t10 * t72 * t459 * t80 - f64x8::splat(5.39442798) * t310 * t464 + f64x8::splat(5.39442798) * t310 * t468 - f64x8::splat(2.69721399) * t274 * t472 - f64x8::splat(2.69721399) * t274 * t475 + f64x8::splat(1.5) * t10 * t480 * t481 - f64x8::splat(1.5) * t368 * t486 + f64x8::splat(2.69721399) * t181 * t269 * t80 + f64x8::splat(0.75) * t84 * t134 * t300 + f64x8::splat(0.89907133) * t10 * t77 * t610 * t80 + f64x8::splat(0.25) * t10 * t135 * t636 - f64x8::splat(5.39442798) * t640 * t641 - f64x8::splat(5.39442798) * t181 * t276 - f64x8::splat(1.5) * t84 * t280 * t282 + f64x8::splat(5.39442798) * t274 * t650;
            let t654 = ((t4).select(f64x8::splat(0.0), t653));
            let tv3rho30 = v_rho * t654 + f64x8::splat(3.0) * t305;
            acc_v3rho3 = tv3rho30;
            let t657 = t353 * t134;
            let t658 = t657 * t144;
            let t661 = t358 * t300;
            let t664 = t471 * t174;
            let t667 = t275 * t386;
            let t670 = t280 * t386;
            let t671 = t670 * t144;
            let t674 = t369 * t300;
            let t677 = t353 * t80;
            let t678 = t677 * t128;
            let t681 = t307 * t269;
            let t691 = -f64x8::splat(1.79814266) * t274 * t658 - f64x8::splat(0.89907133) * t274 * t661 - f64x8::splat(0.89907133) * t274 * t664 - f64x8::splat(1.79814266) * t274 * t667 - f64x8::splat(1.0) * t368 * t671 - f64x8::splat(0.5) * t368 * t674 - f64x8::splat(3.59628532) * t310 * t678 - f64x8::splat(1.79814266) * t310 * t681 + f64x8::splat(0.5) * t84 * t134 * t386 + f64x8::splat(1.79814266) * t181 * t677 - f64x8::splat(3.59628532) * t640 * t311;
            let t698 = t321 * t28;
            let t706 = t151 * t107;
            let t712 = t528 * v_sigma * t98;
            let t713 = t208 * t712;
            let t715 = f64x8::splat(1.0) / t35 / t542;
            let t717 = t715 * t16 * t538;
            let t720 = t34 * t216;
            let t723 = t548 * t720;
            let t726 = t222 * t226;
            let t730 = t152 * t233;
            let t735 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(11097.0) * t505 * t314 * t315 * t193 + f64x8::splat(64.0) / f64x8::splat(1520289.0) * t511 * t34 * t698 - f64x8::splat(4.0) / f64x8::splat(11097.0) * t518 * t223 * t327 * t28 - f64x8::splat(4.0) / f64x8::splat(11097.0) * t203 * t314 * t706 * t28 - f64x8::splat(128.0) / f64x8::splat(208279593.0) * t713 * t717 + f64x8::splat(224.0) / f64x8::splat(506763.0) * t211 * t720 + f64x8::splat(4.0) / f64x8::splat(11097.0) * t221 * t723 - f64x8::splat(10.0) / f64x8::splat(3699.0) * t325 * t103 * t726 - f64x8::splat(14.0) / f64x8::splat(11097.0) * t100 * t103 * t730));
            let t740 = t321 * t55;
            let t752 = t255 * t712;
            let t766 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(11097.0) * t576 * t314 * t315 * t240 + f64x8::splat(64.0) / f64x8::splat(1520289.0) * t582 * t34 * t740 - f64x8::splat(4.0) / f64x8::splat(11097.0) * t588 * t223 * t327 * t55 - f64x8::splat(4.0) / f64x8::splat(11097.0) * t250 * t314 * t706 * t55 - f64x8::splat(128.0) / f64x8::splat(208279593.0) * t752 * t717 + f64x8::splat(224.0) / f64x8::splat(506763.0) * t256 * t720 + f64x8::splat(4.0) / f64x8::splat(11097.0) * t260 * t723 - f64x8::splat(10.0) / f64x8::splat(3699.0) * t325 * t121 * t726 - f64x8::splat(14.0) / f64x8::splat(11097.0) * t100 * t121 * t730));
            let t768 = ((t65).select(f64x8::splat(0.0), t735 + t766));
            let t777 = t616 * t163;
            let t780 = t287 * t353;
            let t785 = t137 * t768;
            let t793 = t72 * t768;
            let t803 = -f64x8::splat(3746.4863903453033) * t777 * t188 + f64x8::splat(1248.8287967817678) * t780 * t128 + f64x8::splat(624.4143983908839) * t373 * t269 - f64x8::splat(124.88287967817678) * t785 - f64x8::splat(894.2243935327947) * t373 * t188 + f64x8::splat(357.6897574131179) * t376 * t128 + f64x8::splat(178.84487870655894) * t168 * t269 - f64x8::splat(44.711219676639736) * t793 - f64x8::splat(42.684415321248) * t168 * t188 + f64x8::splat(21.342207660624) * t380 * t128 + f64x8::splat(10.671103830312) * t170 * t269 - f64x8::splat(3.557034610104) * t75 * t768;
            let t807 = t10 * t72;
            let t808 = t307 * t188;
            let t811 = t128 * t144;
            let t815 = t163 * t280;
            let t816 = t815 * t282;
            let t819 = t467 * t174;
            let t822 = t174 * t144;
            let t823 = t649 * t822;
            let t826 = t479 * t174;
            let t827 = t826 * t282;
            let t830 = -f64x8::splat(1.79814266) * t181 * t359 + f64x8::splat(0.89907133) * t10 * t77 * t768 * t80 - f64x8::splat(1.0) * t84 * t370 - f64x8::splat(1.79814266) * t181 * t365 + f64x8::splat(0.25) * t10 * t135 * t803 + f64x8::splat(5.39442798) * t807 * t808 + f64x8::splat(3.59628532) * t310 * t358 * t811 + f64x8::splat(1.79814266) * t274 * t816 + f64x8::splat(1.79814266) * t310 * t819 + f64x8::splat(3.59628532) * t274 * t823 + f64x8::splat(1.5) * t368 * t827;
            let t832 = ((t4).select(f64x8::splat(0.0), t691 + t830));
            let tv3rho2sigma0 = v_rho * t832 + f64x8::splat(2.0) * t391;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t834 = t393 * t80;
            let t837 = t834 * t128;
            let t840 = t307 * t353;
            let t843 = t393 * t134;
            let t844 = t843 * t144;
            let t847 = t428 * t80;
            let t850 = t847 * t128;
            let t853 = t399 * t28;
            let t857 = t528 * t98;
            let t858 = t208 * t857;
            let t860 = f64x8::splat(1.0) / t35 / t215;
            let t862 = t860 * t16 * t538;
            let t865 = t398 * t321;
            let t872 = t548 * t865;
            let t875 = t403 * t327;
            let t876 = t223 * t875;
            let t879 = t410 * t37;
            let t884 = t411 * t107;
            let t889 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(4.0) / f64x8::splat(506763.0) * t511 * t398 * t853 + f64x8::splat(16.0) / f64x8::splat(69426531.0) * t858 * t862 - f64x8::splat(40.0) / f64x8::splat(506763.0) * t211 * t865 + t518 * t223 * t406 * t28 / f64x8::splat(14796.0) - t221 * t872 / f64x8::splat(7398.0) + t221 * t876 / f64x8::splat(3699.0) - t203 * t314 * t879 * t28 / f64x8::splat(14796.0) - t100 * t103 * t884 / f64x8::splat(3699.0)));
            let t890 = t399 * t55;
            let t894 = t255 * t857;
            let t915 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(4.0) / f64x8::splat(506763.0) * t582 * t398 * t890 + f64x8::splat(16.0) / f64x8::splat(69426531.0) * t894 * t862 - f64x8::splat(40.0) / f64x8::splat(506763.0) * t256 * t865 + t588 * t223 * t406 * t55 / f64x8::splat(14796.0) - t260 * t872 / f64x8::splat(7398.0) + t260 * t876 / f64x8::splat(3699.0) - t250 * t314 * t879 * t55 / f64x8::splat(14796.0) - t100 * t121 * t884 / f64x8::splat(3699.0)));
            let t917 = ((t65).select(f64x8::splat(0.0), t889 + t915));
            let t922 = t428 * t134;
            let t923 = t922 * t144;
            let t928 = t174 * t128;
            let t933 = t657 * t174;
            let t939 = t358 * t386;
            let t945 = t649 * t436;
            let t948 = t479 * t436;
            let t949 = t948 * t144;
            let t952 = t369 * t386;
            let t958 = t275 * t452;
            let t961 = t280 * t452;
            let t962 = t961 * t144;
            let t965 = t616 * t393;
            let t970 = t287 * t428;
            let t973 = t137 * t917;
            let t981 = t72 * t917;
            let t991 = -f64x8::splat(3746.4863903453033) * t965 * t128 + f64x8::splat(1248.8287967817678) * t373 * t353 + f64x8::splat(624.4143983908839) * t970 * t128 - f64x8::splat(124.88287967817678) * t973 - f64x8::splat(894.2243935327947) * t440 * t128 + f64x8::splat(357.6897574131179) * t168 * t353 + f64x8::splat(178.84487870655894) * t442 * t128 - f64x8::splat(44.711219676639736) * t981 - f64x8::splat(42.684415321248) * t444 * t128 + f64x8::splat(21.342207660624) * t170 * t353 + f64x8::splat(10.671103830312) * t446 * t128 - f64x8::splat(3.557034610104) * t75 * t917;
            let t995 = -f64x8::splat(1.79814266) * t274 * t933 + f64x8::splat(3.59628532) * t274 * t815 * t822 - f64x8::splat(1.79814266) * t274 * t939 - f64x8::splat(0.5) * t84 * t280 * t436 + f64x8::splat(1.79814266) * t274 * t945 + f64x8::splat(1.5) * t368 * t949 - f64x8::splat(1.0) * t368 * t952 + f64x8::splat(0.25) * t84 * t134 * t452 - f64x8::splat(0.89907133) * t274 * t958 - f64x8::splat(0.5) * t368 * t962 + f64x8::splat(0.25) * t10 * t135 * t991;
            let t997 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(1.79814266) * t640 * t834 + f64x8::splat(5.39442798) * t807 * t837 - f64x8::splat(3.59628532) * t310 * t840 + f64x8::splat(1.79814266) * t310 * t844 + f64x8::splat(0.89907133) * t181 * t847 - f64x8::splat(1.79814266) * t310 * t850 + f64x8::splat(0.89907133) * t10 * t77 * t917 * t80 - f64x8::splat(0.89907133) * t274 * t923 - f64x8::splat(1.79814266) * t181 * t433 + f64x8::splat(3.59628532) * t310 * t358 * t928 + t995));
            let tv3rhosigma20 = v_rho * t997 + t457;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t999 = t393 * t163;
            let t1004 = t307 * t428;
            let t1007 = t843 * t174;
            let t1011 = t528 * t403 * t98;
            let t1012 = t208 * t1011;
            let t1014 = f64x8::splat(1.0) / t35 / t320;
            let t1015 = t1014 * t16;
            let t1016 = t1015 * t538;
            let t1019 = t410 * t23;
            let t1020 = t1019 * t399;
            let t1023 = t548 * t1020;
            let t1026 = f64x8::splat(1.0) / t529;
            let t1027 = t1026 * t405;
            let t1028 = t223 * t1027;
            let t1031 = t33 * t529;
            let t1032 = f64x8::splat(1.0) / t1031;
            let t1033 = t39 * t1032;
            let t1034 = t1033 * t37;
            let t1039 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(23142177.0) * t1012 * t1016 - t211 * t1020 / f64x8::splat(56307.0) + t221 * t1023 / f64x8::splat(19728.0) + t221 * t1028 / f64x8::splat(3288.0) - t100 * t103 * t1034 / f64x8::splat(3288.0)));
            let t1040 = t255 * t1011;
            let t1053 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(23142177.0) * t1040 * t1016 - t256 * t1020 / f64x8::splat(56307.0) + t260 * t1023 / f64x8::splat(19728.0) + t260 * t1028 / f64x8::splat(3288.0) - t100 * t121 * t1034 / f64x8::splat(3288.0)));
            let t1055 = ((t65).select(f64x8::splat(0.0), t1039 + t1053));
            let t1060 = t922 * t174;
            let t1063 = t815 * t436;
            let t1066 = t358 * t452;
            let t1069 = t436 * t174;
            let t1073 = t369 * t452;
            let t1076 = t616 * t999;
            let t1080 = t137 * t1055;
            let t1082 = t287 * t999;
            let t1086 = t72 * t1055;
            let t1094 = -f64x8::splat(3746.4863903453033) * t1076 + f64x8::splat(1873.2431951726517) * t373 * t428 - f64x8::splat(124.88287967817678) * t1080 - f64x8::splat(894.2243935327947) * t1082 + f64x8::splat(536.5346361196769) * t168 * t428 - f64x8::splat(44.711219676639736) * t1086 - f64x8::splat(42.684415321248) * t137 * t999 + f64x8::splat(32.013311490936) * t170 * t428 - f64x8::splat(3.557034610104) * t75 * t1055;
            let t1099 = ((t4).select(f64x8::splat(0.0), f64x8::splat(5.39442798) * t10 * t72 * t999 * t80 - f64x8::splat(5.39442798) * t310 * t1004 + f64x8::splat(5.39442798) * t310 * t1007 + f64x8::splat(0.89907133) * t10 * t77 * t1055 * t80 - f64x8::splat(2.69721399) * t274 * t1060 + f64x8::splat(5.39442798) * t274 * t1063 - f64x8::splat(2.69721399) * t274 * t1066 + f64x8::splat(1.5) * t10 * t480 * t1069 - f64x8::splat(1.5) * t368 * t1073 + f64x8::splat(0.25) * t10 * t135 * t1094));
            let tv3sigma30 = v_rho * t1099;
            acc_v3sigma3 = tv3sigma30;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho3.into(); v3rho3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho2sigma.into(); v3rho2sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rhosigma2.into(); v3rhosigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3sigma3.into(); v3sigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
