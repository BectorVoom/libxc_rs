//! GGA_C_OP_PBE kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_pbe.c`
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
pub fn gga_c_op_pbe_kxc_unpol(
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
            let t20 = t16 / t18;
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
            let t33 = f64x8::splat(M_CBRT6);
            let t34 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t35 = (simd::cbrt(t34));
            let t36 = t35 * t35;
            let t37 = f64x8::splat(1.0) / t36;
            let t38 = t33 * t37;
            let t39 = t23 * t23;
            let t41 = v_rho * v_rho;
            let t42 = (simd::cbrt(v_rho));
            let t43 = t42 * t42;
            let t45 = f64x8::splat(1.0) / t43 / t41;
            let t49 = f64x8::splat(0.804) + f64x8::splat(0.009146457198521547) * t38 * v_sigma * t39 * t45;
            let t52 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t49;
            let t53 = f64x8::splat(1.0) / t52;
            let t57 = ((t14).select(f64x8::splat(0.0), t22 * t23 * t31 * t53 / f64x8::splat(9.0)));
            let t61 = (t25 * v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t62 = ((t26).select(t5, (t24).select(t6, -t7)));
            let t63 = f64x8::splat(1.0) + t62;
            let t64 = t63 * v_rho;
            let t65 = (simd::cbrt(t64));
            let t66 = f64x8::splat(1.0) / t65;
            let t71 = ((t61).select(f64x8::splat(0.0), t22 * t23 * t66 * t53 / f64x8::splat(9.0)));
            let t72 = t57 + t71;
            let t73 = (t72).simd_eq(f64x8::splat(0.0));
            let t74 = ((t73).select(f64x8::splat(f64::EPSILON), t72));
            let t77 = f64x8::splat(3.61925846) / t74 + f64x8::splat(0.5764);
            let t78 = t74 * t74;
            let t79 = t78 * t78;
            let t80 = f64x8::splat(1.0) / t79;
            let t82 = t78 * t74;
            let t83 = f64x8::splat(1.0) / t82;
            let t85 = f64x8::splat(1.0) / t78;
            let t87 = f64x8::splat(32.02615087407435) * t80 + f64x8::splat(15.19118443242906) * t83 + f64x8::splat(1.801312286343) * t85;
            let t88 = f64x8::splat(1.0) / t87;
            let tzk0 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t10 * t77 * t88));
            acc_zk = tzk0;
            let t92 = t9 * t77;
            let t96 = f64x8::splat(1.0) / t30 / t29;
            let t102 = t21 * t31;
            let t103 = t52 * t52;
            let t104 = f64x8::splat(1.0) / t103;
            let t106 = t20 * t102 * t104;
            let t107 = t49 * t49;
            let t108 = f64x8::splat(1.0) / t107;
            let t109 = t108 * t33;
            let t110 = t37 * v_sigma;
            let t111 = t41 * v_rho;
            let t113 = f64x8::splat(1.0) / t43 / t111;
            let t114 = t110 * t113;
            let t115 = t109 * t114;
            let t119 = ((t14).select(f64x8::splat(0.0), -t22 * t23 * t96 * t53 * t28 / f64x8::splat(27.0) + f64x8::splat(0.003503654089741928) * t106 * t115));
            let t121 = f64x8::splat(1.0) / t65 / t64;
            let t127 = t21 * t66;
            let t129 = t20 * t127 * t104;
            let t133 = ((t61).select(f64x8::splat(0.0), -t22 * t23 * t121 * t53 * t63 / f64x8::splat(27.0) + f64x8::splat(0.003503654089741928) * t129 * t115));
            let t135 = ((t73).select(f64x8::splat(0.0), t119 + t133));
            let t140 = t87 * t87;
            let t141 = f64x8::splat(1.0) / t140;
            let t142 = t77 * t141;
            let t144 = f64x8::splat(1.0) / t79 / t74;
            let t145 = t144 * t135;
            let t147 = t80 * t135;
            let t151 = -f64x8::splat(128.1046034962974) * t145 - f64x8::splat(45.57355329728718) * t147 - f64x8::splat(3.602624572686) * t83 * t135;
            let t156 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t92 * t88 + f64x8::splat(0.904814615) * t10 * t85 * t135 * t88 + f64x8::splat(0.25) * t10 * t142 * t151));
            let tvrho0 = v_rho * t156 + tzk0;
            acc_vrho = tvrho0;
            let t158 = t20 * t102;
            let t159 = t104 * t108;
            let t161 = t159 * t38 * t45;
            let t164 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(0.001313870283653223) * t158 * t161));
            let t165 = t20 * t127;
            let t168 = ((t61).select(f64x8::splat(0.0), -f64x8::splat(0.001313870283653223) * t165 * t161));
            let t170 = ((t73).select(f64x8::splat(0.0), t164 + t168));
            let t175 = t144 * t170;
            let t177 = t80 * t170;
            let t179 = t83 * t170;
            let t181 = -f64x8::splat(128.1046034962974) * t175 - f64x8::splat(45.57355329728718) * t177 - f64x8::splat(3.602624572686) * t179;
            let t186 = ((t4).select(f64x8::splat(0.0), f64x8::splat(0.904814615) * t10 * t85 * t170 * t88 + f64x8::splat(0.25) * t10 * t142 * t181));
            let tvsigma0 = v_rho * t186;
            acc_vsigma = tvsigma0;
            let t188 = t9 * t85;
            let t189 = t135 * t88;
            let t195 = t135 * t135;
            let t200 = t28 * t28;
            let t203 = f64x8::splat(1.0) / t30 / t200 / t41;
            let t209 = t21 * t96;
            let t211 = t20 * t209 * t104;
            let t213 = t28 * t108 * t33;
            let t218 = f64x8::splat(1.0) / t103 / t52;
            let t220 = t20 * t102 * t218;
            let t221 = t107 * t107;
            let t222 = f64x8::splat(1.0) / t221;
            let t223 = t33 * t33;
            let t224 = t222 * t223;
            let t226 = f64x8::splat(1.0) / t35 / t34;
            let t227 = t224 * t226;
            let t228 = v_sigma * v_sigma;
            let t229 = t41 * t41;
            let t232 = f64x8::splat(1.0) / t42 / t229 / t111;
            let t234 = t228 * t232 * t39;
            let t235 = t227 * t234;
            let t238 = t107 * t49;
            let t239 = f64x8::splat(1.0) / t238;
            let t240 = t239 * t223;
            let t241 = t240 * t226;
            let t242 = t241 * t234;
            let t246 = f64x8::splat(1.0) / t43 / t229;
            let t247 = t110 * t246;
            let t248 = t109 * t247;
            let t252 = ((t14).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(81.0) * t22 * t23 * t203 * t53 * t200 - f64x8::splat(0.0023357693931612853) * t211 * t213 * t114 + f64x8::splat(0.00011048032782508804) * t220 * t235 + f64x8::splat(0.00017091211824133074) * t106 * t242 - f64x8::splat(0.012846731662387069) * t106 * t248));
            let t253 = t63 * t63;
            let t256 = f64x8::splat(1.0) / t65 / t253 / t41;
            let t262 = t21 * t121;
            let t264 = t20 * t262 * t104;
            let t266 = t63 * t108 * t33;
            let t271 = t20 * t127 * t218;
            let t279 = ((t61).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(81.0) * t22 * t23 * t256 * t53 * t253 - f64x8::splat(0.0023357693931612853) * t264 * t266 * t114 + f64x8::splat(0.00011048032782508804) * t271 * t235 + f64x8::splat(0.00017091211824133074) * t129 * t242 - f64x8::splat(0.012846731662387069) * t129 * t248));
            let t281 = ((t73).select(f64x8::splat(0.0), t252 + t279));
            let t286 = t10 * t85;
            let t287 = t135 * t141;
            let t288 = t287 * t151;
            let t292 = f64x8::splat(1.0) / t140 / t87;
            let t293 = t77 * t292;
            let t294 = t151 * t151;
            let t299 = f64x8::splat(1.0) / t79 / t78;
            let t300 = t299 * t195;
            let t304 = t144 * t195;
            let t312 = f64x8::splat(640.5230174814869) * t300 - f64x8::splat(128.1046034962974) * t144 * t281 + f64x8::splat(182.2942131891487) * t304 - f64x8::splat(45.57355329728718) * t80 * t281 + f64x8::splat(10.807873718058) * t80 * t195 - f64x8::splat(3.602624572686) * t83 * t281;
            let t317 = ((t4).select(f64x8::splat(0.0), f64x8::splat(1.80962923) * t188 * t189 + f64x8::splat(0.5) * t92 * t141 * t151 - f64x8::splat(1.80962923) * t10 * t83 * t195 * t88 + f64x8::splat(0.904814615) * t10 * t85 * t281 * t88 - f64x8::splat(1.80962923) * t286 * t288 - f64x8::splat(0.5) * t10 * t293 * t294 + f64x8::splat(0.25) * t10 * t142 * t312));
            let tv2rho20 = v_rho * t317 + f64x8::splat(2.0) * t156;
            acc_v2rho2 = tv2rho20;
            let t319 = t170 * t88;
            let t322 = t10 * t83;
            let t323 = t319 * t135;
            let t326 = t37 * t45;
            let t331 = t229 * t41;
            let t333 = f64x8::splat(1.0) / t42 / t331;
            let t335 = t333 * v_sigma * t39;
            let t336 = t227 * t335;
            let t339 = t241 * t335;
            let t343 = t159 * t38 * t113;
            let t347 = ((t14).select(f64x8::splat(0.0), f64x8::splat(0.000437956761217741) * t211 * t109 * t326 * t28 - f64x8::splat(4.1430122934408016e-05) * t220 * t336 - f64x8::splat(6.409204434049903e-05) * t106 * t339 + f64x8::splat(0.003503654089741928) * t158 * t343));
            let t359 = ((t61).select(f64x8::splat(0.0), f64x8::splat(0.000437956761217741) * t264 * t109 * t326 * t63 - f64x8::splat(4.1430122934408016e-05) * t271 * t336 - f64x8::splat(6.409204434049903e-05) * t129 * t339 + f64x8::splat(0.003503654089741928) * t165 * t343));
            let t361 = ((t73).select(f64x8::splat(0.0), t347 + t359));
            let t366 = t170 * t141;
            let t367 = t366 * t151;
            let t373 = t287 * t181;
            let t376 = t10 * t77;
            let t377 = t292 * t181;
            let t378 = t377 * t151;
            let t381 = t299 * t170;
            let t384 = t144 * t361;
            let t388 = t80 * t361;
            let t394 = f64x8::splat(640.5230174814869) * t381 * t135 - f64x8::splat(128.1046034962974) * t384 + f64x8::splat(182.2942131891487) * t175 * t135 - f64x8::splat(45.57355329728718) * t388 + f64x8::splat(10.807873718058) * t177 * t135 - f64x8::splat(3.602624572686) * t83 * t361;
            let t399 = ((t4).select(f64x8::splat(0.0), f64x8::splat(0.904814615) * t188 * t319 - f64x8::splat(1.80962923) * t322 * t323 + f64x8::splat(0.904814615) * t10 * t85 * t361 * t88 - f64x8::splat(0.904814615) * t286 * t367 + f64x8::splat(0.25) * t92 * t141 * t181 - f64x8::splat(0.904814615) * t286 * t373 - f64x8::splat(0.5) * t376 * t378 + f64x8::splat(0.25) * t10 * t142 * t394));
            let tv2rhosigma0 = v_rho * t399 + t186;
            acc_v2rhosigma = tv2rhosigma0;
            let t401 = t170 * t170;
            let t406 = t229 * v_rho;
            let t408 = f64x8::splat(1.0) / t42 / t406;
            let t410 = t226 * t408 * t39;
            let t411 = t224 * t410;
            let t414 = t240 * t410;
            let t418 = ((t14).select(f64x8::splat(0.0), f64x8::splat(1.5536296100403008e-05) * t220 * t411 + f64x8::splat(2.4034516627687134e-05) * t106 * t414));
            let t424 = ((t61).select(f64x8::splat(0.0), f64x8::splat(1.5536296100403008e-05) * t271 * t411 + f64x8::splat(2.4034516627687134e-05) * t129 * t414));
            let t426 = ((t73).select(f64x8::splat(0.0), t418 + t424));
            let t431 = t366 * t181;
            let t434 = t181 * t181;
            let t438 = t299 * t401;
            let t440 = t144 * t426;
            let t442 = t144 * t401;
            let t444 = t80 * t426;
            let t450 = f64x8::splat(640.5230174814869) * t438 - f64x8::splat(128.1046034962974) * t440 + f64x8::splat(182.2942131891487) * t442 - f64x8::splat(45.57355329728718) * t444 + f64x8::splat(10.807873718058) * t80 * t401 - f64x8::splat(3.602624572686) * t83 * t426;
            let t455 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(1.80962923) * t10 * t83 * t401 * t88 + f64x8::splat(0.904814615) * t10 * t85 * t426 * t88 - f64x8::splat(1.80962923) * t286 * t431 - f64x8::splat(0.5) * t10 * t293 * t434 + f64x8::splat(0.25) * t10 * t142 * t450));
            let tv2sigma20 = v_rho * t455;
            acc_v2sigma2 = tv2sigma20;
            let t457 = t135 * t292;
            let t458 = t457 * t294;
            let t461 = t140 * t140;
            let t462 = f64x8::splat(1.0) / t461;
            let t463 = t77 * t462;
            let t464 = t294 * t151;
            let t468 = t292 * t151;
            let t469 = t468 * t312;
            let t472 = t287 * t312;
            let t475 = t281 * t141;
            let t476 = t475 * t151;
            let t479 = t195 * t135;
            let t484 = t189 * t281;
            let t487 = t195 * t141;
            let t488 = t487 * t151;
            let t492 = f64x8::splat(1.0) / t79 / t82;
            let t495 = t299 * t135;
            let t498 = t200 * t28;
            let t501 = f64x8::splat(1.0) / t30 / t498 / t111;
            let t507 = t21 * t203;
            let t509 = t20 * t507 * t104;
            let t511 = t200 * t108 * t33;
            let t515 = t96 * t218;
            let t517 = t22 * t515 * t28;
            let t520 = t96 * t104;
            let t522 = t22 * t520 * t28;
            let t528 = t103 * t103;
            let t529 = f64x8::splat(1.0) / t528;
            let t531 = f64x8::splat(1.0) / t221 / t107;
            let t532 = t529 * t531;
            let t533 = t228 * v_sigma;
            let t534 = t229 * t229;
            let t535 = t534 * t111;
            let t536 = f64x8::splat(1.0) / t535;
            let t538 = t533 * t536 * t23;
            let t539 = t532 * t538;
            let t543 = f64x8::splat(1.0) / t221 / t49;
            let t544 = t218 * t543;
            let t545 = t544 * t538;
            let t549 = f64x8::splat(1.0) / t42 / t534;
            let t551 = t228 * t549 * t39;
            let t552 = t227 * t551;
            let t555 = t104 * t222;
            let t556 = t555 * t538;
            let t559 = t241 * t551;
            let t563 = f64x8::splat(1.0) / t43 / t406;
            let t564 = t110 * t563;
            let t565 = t109 * t564;
            let t568 = -f64x8::splat(28.0) / f64x8::splat(243.0) * t22 * t23 * t501 * t53 * t498 + f64x8::splat(0.004671538786322571) * t509 * t511 * t114 - f64x8::splat(0.00011048032782508804) * t517 * t235 - f64x8::splat(0.00017091211824133074) * t522 * t242 + f64x8::splat(0.012846731662387069) * t211 * t213 * t247 + f64x8::splat(6.437566086127966e-07) * t158 * t539 + f64x8::splat(1.9917718887304666e-06) * t158 * t545 - f64x8::splat(0.0012152836060759684) * t220 * t552 + f64x8::splat(1.5406270023718987e-06) * t158 * t556 - f64x8::splat(0.001880033300654638) * t106 * t559 + f64x8::splat(0.05995141442447299) * t106 * t565;
            let t569 = ((t14).select(f64x8::splat(0.0), t568));
            let t570 = t253 * t63;
            let t573 = f64x8::splat(1.0) / t65 / t570 / t111;
            let t579 = t21 * t256;
            let t581 = t20 * t579 * t104;
            let t583 = t253 * t108 * t33;
            let t587 = t121 * t218;
            let t589 = t22 * t587 * t63;
            let t592 = t121 * t104;
            let t594 = t22 * t592 * t63;
            let t612 = -f64x8::splat(28.0) / f64x8::splat(243.0) * t22 * t23 * t573 * t53 * t570 + f64x8::splat(0.004671538786322571) * t581 * t583 * t114 - f64x8::splat(0.00011048032782508804) * t589 * t235 - f64x8::splat(0.00017091211824133074) * t594 * t242 + f64x8::splat(0.012846731662387069) * t264 * t266 * t247 + f64x8::splat(6.437566086127966e-07) * t165 * t539 + f64x8::splat(1.9917718887304666e-06) * t165 * t545 - f64x8::splat(0.0012152836060759684) * t271 * t552 + f64x8::splat(1.5406270023718987e-06) * t165 * t556 - f64x8::splat(0.001880033300654638) * t129 * t559 + f64x8::splat(0.05995141442447299) * t129 * t565;
            let t613 = ((t61).select(f64x8::splat(0.0), t612));
            let t615 = ((t73).select(f64x8::splat(0.0), t569 + t613));
            let t630 = -f64x8::splat(3843.138104888921) * t492 * t479 + f64x8::splat(1921.5690524444606) * t495 * t281 - f64x8::splat(128.1046034962974) * t144 * t615 - f64x8::splat(911.4710659457436) * t299 * t479 + f64x8::splat(546.8826395674462) * t145 * t281 - f64x8::splat(45.57355329728718) * t80 * t615 - f64x8::splat(43.231494872232) * t144 * t479 + f64x8::splat(32.423621154174) * t147 * t281 - f64x8::splat(3.602624572686) * t83 * t615;
            let t641 = t9 * t83;
            let t642 = t195 * t88;
            let t653 = f64x8::splat(5.42888769) * t286 * t458 + f64x8::splat(1.5) * t10 * t463 * t464 - f64x8::splat(1.5) * t376 * t469 - f64x8::splat(2.714443845) * t286 * t472 - f64x8::splat(2.714443845) * t286 * t476 + f64x8::splat(5.42888769) * t10 * t80 * t479 * t88 - f64x8::splat(5.42888769) * t322 * t484 + f64x8::splat(5.42888769) * t322 * t488 + f64x8::splat(0.25) * t10 * t142 * t630 + f64x8::splat(0.904814615) * t10 * t85 * t615 * t88 - f64x8::splat(1.5) * t92 * t292 * t294 - f64x8::splat(5.42888769) * t641 * t642 - f64x8::splat(5.42888769) * t188 * t288 + f64x8::splat(2.714443845) * t188 * t281 * t88 + f64x8::splat(0.75) * t92 * t141 * t312;
            let t654 = ((t4).select(f64x8::splat(0.0), t653));
            let tv3rho30 = v_rho * t654 + f64x8::splat(3.0) * t317;
            acc_v3rho3 = tv3rho30;
            let t657 = t462 * t181;
            let t658 = t657 * t294;
            let t661 = t487 * t181;
            let t664 = t181 * t151;
            let t665 = t457 * t664;
            let t668 = t377 * t312;
            let t671 = t292 * t394;
            let t672 = t671 * t151;
            let t675 = t475 * t181;
            let t678 = t287 * t394;
            let t681 = t366 * t312;
            let t684 = t361 * t141;
            let t685 = t684 * t151;
            let t688 = t170 * t292;
            let t689 = t688 * t294;
            let t692 = t135 * t151;
            let t696 = f64x8::splat(1.5) * t376 * t658 + f64x8::splat(1.80962923) * t322 * t661 + f64x8::splat(3.61925846) * t286 * t665 - f64x8::splat(0.5) * t376 * t668 - f64x8::splat(1.0) * t376 * t672 - f64x8::splat(0.904814615) * t286 * t675 - f64x8::splat(1.80962923) * t286 * t678 - f64x8::splat(0.904814615) * t286 * t681 - f64x8::splat(1.80962923) * t286 * t685 + f64x8::splat(1.80962923) * t286 * t689 + f64x8::splat(3.61925846) * t322 * t366 * t692;
            let t697 = t10 * t80;
            let t698 = t319 * t195;
            let t701 = t492 * t170;
            let t704 = t299 * t361;
            let t714 = t22 * t515 * t222;
            let t715 = t223 * t226;
            let t716 = t715 * t333;
            let t718 = t28 * v_sigma * t39;
            let t719 = t716 * t718;
            let t723 = t22 * t520 * t239;
            let t726 = t37 * t113;
            let t731 = t534 * t41;
            let t732 = f64x8::splat(1.0) / t731;
            let t734 = t732 * t228 * t23;
            let t735 = t532 * t734;
            let t738 = t544 * t734;
            let t742 = t232 * v_sigma * t39;
            let t743 = t227 * t742;
            let t746 = t555 * t734;
            let t749 = t241 * t742;
            let t753 = t159 * t38 * t246;
            let t757 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(0.0005839423482903213) * t509 * t109 * t326 * t200 + f64x8::splat(2.762008195627201e-05) * t714 * t719 + f64x8::splat(4.2728029560332685e-05) * t723 * t719 - f64x8::splat(0.0023357693931612853) * t211 * t109 * t726 * t28 - f64x8::splat(2.4140872822979875e-07) * t158 * t735 - f64x8::splat(7.46914458273925e-07) * t158 * t738 + f64x8::splat(0.00037287110640967213) * t220 * t743 - f64x8::splat(5.77735125889462e-07) * t158 * t746 + f64x8::splat(0.0005768283990644912) * t106 * t749 - f64x8::splat(0.012846731662387069) * t158 * t753));
            let t763 = t22 * t587 * t222;
            let t765 = t63 * v_sigma * t39;
            let t766 = t716 * t765;
            let t770 = t22 * t592 * t239;
            let t790 = ((t61).select(f64x8::splat(0.0), -f64x8::splat(0.0005839423482903213) * t581 * t109 * t326 * t253 + f64x8::splat(2.762008195627201e-05) * t763 * t766 + f64x8::splat(4.2728029560332685e-05) * t770 * t766 - f64x8::splat(0.0023357693931612853) * t264 * t109 * t726 * t63 - f64x8::splat(2.4140872822979875e-07) * t165 * t735 - f64x8::splat(7.46914458273925e-07) * t165 * t738 + f64x8::splat(0.00037287110640967213) * t271 * t743 - f64x8::splat(5.77735125889462e-07) * t165 * t746 + f64x8::splat(0.0005768283990644912) * t129 * t749 - f64x8::splat(0.012846731662387069) * t165 * t753));
            let t792 = ((t73).select(f64x8::splat(0.0), t757 + t790));
            let t793 = t144 * t792;
            let t801 = t80 * t792;
            let t811 = -f64x8::splat(3843.138104888921) * t701 * t195 + f64x8::splat(1281.0460349629739) * t704 * t135 + f64x8::splat(640.5230174814869) * t381 * t281 - f64x8::splat(128.1046034962974) * t793 - f64x8::splat(911.4710659457436) * t381 * t195 + f64x8::splat(364.5884263782974) * t384 * t135 + f64x8::splat(182.2942131891487) * t175 * t281 - f64x8::splat(45.57355329728718) * t801 - f64x8::splat(43.231494872232) * t175 * t195 + f64x8::splat(21.615747436116) * t388 * t135 + f64x8::splat(10.807873718058) * t177 * t281 - f64x8::splat(3.602624572686) * t83 * t792;
            let t827 = t361 * t88;
            let t828 = t827 * t135;
            let t831 = t319 * t281;
            let t839 = f64x8::splat(5.42888769) * t697 * t698 + f64x8::splat(0.25) * t10 * t142 * t811 - f64x8::splat(1.80962923) * t188 * t373 - f64x8::splat(1.0) * t92 * t378 + f64x8::splat(0.904814615) * t10 * t85 * t792 * t88 - f64x8::splat(3.61925846) * t641 * t323 - f64x8::splat(1.80962923) * t188 * t367 - f64x8::splat(3.61925846) * t322 * t828 - f64x8::splat(1.80962923) * t322 * t831 + f64x8::splat(1.80962923) * t188 * t827 + f64x8::splat(0.5) * t92 * t141 * t394;
            let t841 = ((t4).select(f64x8::splat(0.0), t696 + t839));
            let tv3rho2sigma0 = v_rho * t841 + f64x8::splat(2.0) * t399;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t843 = t401 * t88;
            let t846 = t843 * t135;
            let t849 = t319 * t361;
            let t852 = t401 * t141;
            let t853 = t852 * t151;
            let t856 = t426 * t88;
            let t859 = t856 * t135;
            let t863 = t20 * t209 * t218;
            let t864 = t408 * t39;
            let t865 = t864 * t28;
            let t869 = t534 * v_rho;
            let t870 = f64x8::splat(1.0) / t869;
            let t871 = t870 * t23;
            let t872 = t871 * v_sigma;
            let t873 = t532 * t872;
            let t876 = t544 * t872;
            let t880 = t226 * t333 * t39;
            let t881 = t224 * t880;
            let t887 = t555 * t872;
            let t890 = t240 * t880;
            let t894 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(5.178765366801002e-06) * t863 * t227 * t865 + f64x8::splat(9.052827308617452e-08) * t158 * t873 + f64x8::splat(2.8009292185272186e-07) * t158 * t876 - f64x8::splat(8.286024586881603e-05) * t220 * t881 - f64x8::splat(8.011505542562378e-06) * t211 * t241 * t865 + f64x8::splat(2.1665067220854825e-07) * t158 * t887 - f64x8::splat(0.00012818408868099806) * t106 * t890));
            let t896 = t20 * t262 * t218;
            let t897 = t864 * t63;
            let t915 = ((t61).select(f64x8::splat(0.0), -f64x8::splat(5.178765366801002e-06) * t896 * t227 * t897 + f64x8::splat(9.052827308617452e-08) * t165 * t873 + f64x8::splat(2.8009292185272186e-07) * t165 * t876 - f64x8::splat(8.286024586881603e-05) * t271 * t881 - f64x8::splat(8.011505542562378e-06) * t264 * t241 * t897 + f64x8::splat(2.1665067220854825e-07) * t165 * t887 - f64x8::splat(0.00012818408868099806) * t129 * t890));
            let t917 = ((t73).select(f64x8::splat(0.0), t894 + t915));
            let t922 = t426 * t141;
            let t923 = t922 * t151;
            let t928 = t181 * t135;
            let t933 = t684 * t181;
            let t939 = t366 * t394;
            let t945 = t457 * t434;
            let t948 = t462 * t434;
            let t949 = t948 * t151;
            let t952 = t377 * t394;
            let t958 = t287 * t450;
            let t961 = t292 * t450;
            let t962 = t961 * t151;
            let t965 = t492 * t401;
            let t970 = t299 * t426;
            let t973 = t144 * t917;
            let t981 = t80 * t917;
            let t991 = -f64x8::splat(3843.138104888921) * t965 * t135 + f64x8::splat(1281.0460349629739) * t381 * t361 + f64x8::splat(640.5230174814869) * t970 * t135 - f64x8::splat(128.1046034962974) * t973 - f64x8::splat(911.4710659457436) * t438 * t135 + f64x8::splat(364.5884263782974) * t175 * t361 + f64x8::splat(182.2942131891487) * t440 * t135 - f64x8::splat(45.57355329728718) * t981 - f64x8::splat(43.231494872232) * t442 * t135 + f64x8::splat(21.615747436116) * t177 * t361 + f64x8::splat(10.807873718058) * t444 * t135 - f64x8::splat(3.602624572686) * t83 * t917;
            let t995 = -f64x8::splat(1.80962923) * t286 * t933 + f64x8::splat(3.61925846) * t286 * t688 * t664 - f64x8::splat(1.80962923) * t286 * t939 - f64x8::splat(0.5) * t92 * t292 * t434 + f64x8::splat(1.80962923) * t286 * t945 + f64x8::splat(1.5) * t376 * t949 - f64x8::splat(1.0) * t376 * t952 + f64x8::splat(0.25) * t92 * t141 * t450 - f64x8::splat(0.904814615) * t286 * t958 - f64x8::splat(0.5) * t376 * t962 + f64x8::splat(0.25) * t10 * t142 * t991;
            let t997 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(1.80962923) * t641 * t843 + f64x8::splat(5.42888769) * t697 * t846 - f64x8::splat(3.61925846) * t322 * t849 + f64x8::splat(1.80962923) * t322 * t853 + f64x8::splat(0.904814615) * t188 * t856 - f64x8::splat(1.80962923) * t322 * t859 + f64x8::splat(0.904814615) * t10 * t85 * t917 * t88 - f64x8::splat(0.904814615) * t286 * t923 - f64x8::splat(1.80962923) * t188 * t431 + f64x8::splat(3.61925846) * t322 * t366 * t928 + t995));
            let tv3rhosigma20 = v_rho * t997 + t455;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t999 = t401 * t170;
            let t1004 = t319 * t426;
            let t1007 = t852 * t181;
            let t1011 = f64x8::splat(1.0) / t534 * t23;
            let t1012 = t532 * t1011;
            let t1015 = t544 * t1011;
            let t1018 = t555 * t1011;
            let t1022 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(3.3948102407315447e-08) * t158 * t1012 - f64x8::splat(1.050348456947707e-07) * t158 * t1015 - f64x8::splat(8.124400207820559e-08) * t158 * t1018));
            let t1030 = ((t61).select(f64x8::splat(0.0), -f64x8::splat(3.3948102407315447e-08) * t165 * t1012 - f64x8::splat(1.050348456947707e-07) * t165 * t1015 - f64x8::splat(8.124400207820559e-08) * t165 * t1018));
            let t1032 = ((t73).select(f64x8::splat(0.0), t1022 + t1030));
            let t1037 = t922 * t181;
            let t1040 = t688 * t434;
            let t1043 = t366 * t450;
            let t1046 = t434 * t181;
            let t1050 = t377 * t450;
            let t1053 = t492 * t999;
            let t1057 = t144 * t1032;
            let t1059 = t299 * t999;
            let t1063 = t80 * t1032;
            let t1071 = -f64x8::splat(3843.138104888921) * t1053 + f64x8::splat(1921.5690524444606) * t381 * t426 - f64x8::splat(128.1046034962974) * t1057 - f64x8::splat(911.4710659457436) * t1059 + f64x8::splat(546.8826395674462) * t175 * t426 - f64x8::splat(45.57355329728718) * t1063 - f64x8::splat(43.231494872232) * t144 * t999 + f64x8::splat(32.423621154174) * t177 * t426 - f64x8::splat(3.602624572686) * t83 * t1032;
            let t1076 = ((t4).select(f64x8::splat(0.0), f64x8::splat(5.42888769) * t10 * t80 * t999 * t88 - f64x8::splat(5.42888769) * t322 * t1004 + f64x8::splat(5.42888769) * t322 * t1007 + f64x8::splat(0.904814615) * t10 * t85 * t1032 * t88 - f64x8::splat(2.714443845) * t286 * t1037 + f64x8::splat(5.42888769) * t286 * t1040 - f64x8::splat(2.714443845) * t286 * t1043 + f64x8::splat(1.5) * t10 * t463 * t1046 - f64x8::splat(1.5) * t376 * t1050 + f64x8::splat(0.25) * t10 * t142 * t1071));
            let tv3sigma30 = v_rho * t1076;
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
