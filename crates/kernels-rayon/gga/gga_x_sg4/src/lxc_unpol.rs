//! GGA_X_SG4 lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sg4.c`
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
pub fn gga_x_sg4_lxc_unpol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
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
        let mut acc_v4rho4 = V_ZERO;
        let mut acc_v4rho3sigma = V_ZERO;
        let mut acc_v4rho2sigma2 = V_ZERO;
        let mut acc_v4rhosigma3 = V_ZERO;
        let mut acc_v4sigma4 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = f64x8::splat(M_CBRT6);
            let t21 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t22 = (simd::cbrt(t21));
            let t23 = t22 * t22;
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t20 * t24;
            let t26 = f64x8::splat(M_CBRT2);
            let t27 = t26 * t26;
            let t28 = v_sigma * t27;
            let t29 = v_rho * v_rho;
            let t30 = t18 * t18;
            let t32 = f64x8::splat(1.0) / t30 / t29;
            let t34 = t25 * t28 * t32;
            let t36 = f64x8::splat(1.0) - f64x8::splat(0.0031233982573039467) * t34;
            let t37 = t20 * t20;
            let t38 = t21 * t21;
            let t39 = t38 * t21;
            let t41 = f64x8::splat(1.0) / t22 / t39;
            let t42 = t37 * t41;
            let t43 = v_sigma * v_sigma;
            let t44 = t43 * t43;
            let t45 = t44 * v_sigma;
            let t47 = t29 * t29;
            let t48 = t47 * v_rho;
            let t49 = t47 * t47;
            let t50 = t49 * t48;
            let t52 = f64x8::splat(1.0) / t18 / t50;
            let t56 = f64x8::splat(1.0) - f64x8::splat(1.426849132767203e-11) * t42 * t45 * t26 * t52;
            let t57 = f64x8::splat(1.0) / t56;
            let t61 = f64x8::splat(1.0) + f64x8::splat(0.03727064220183486) * t34;
            let t64 = f64x8::splat(1.804) - f64x8::splat(0.5602871794871794) * t36 * t57 - f64x8::splat(0.2437128205128205) / t61;
            let t68 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t64));
            let tzk0 = f64x8::splat(2.0) * t68;
            acc_zk = tzk0;
            let t70 = t17 / t30;
            let t74 = t25 * v_sigma;
            let t75 = t29 * v_rho;
            let t77 = f64x8::splat(1.0) / t30 / t75;
            let t79 = t27 * t77 * t57;
            let t82 = t56 * t56;
            let t83 = f64x8::splat(1.0) / t82;
            let t85 = t36 * t83 * t37;
            let t86 = t41 * t45;
            let t87 = t47 * t29;
            let t88 = t49 * t87;
            let t91 = t26 / t18 / t88;
            let t95 = t61 * t61;
            let t97 = f64x8::splat(1.0) / t95 * t20;
            let t98 = t97 * t24;
            let t102 = -f64x8::splat(0.004666666666666667) * t74 * t79 + f64x8::splat(1.0659270348691523e-10) * t85 * t86 * t91 - f64x8::splat(0.02422222222222222) * t98 * t28 * t77;
            let t107 = ((t2).select(f64x8::splat(0.0), -t6 * t70 * t64 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t102));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t107 + f64x8::splat(2.0) * t68;
            acc_vrho = tvrho0;
            let t114 = t41 * t44;
            let t115 = t26 * t52;
            let t119 = t24 * t27;
            let t123 = f64x8::splat(0.00175) * t25 * t27 * t32 * t57 - f64x8::splat(3.997226380759321e-11) * t85 * t114 * t115 + f64x8::splat(0.009083333333333334) * t97 * t119 * t32;
            let t127 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t123));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t127;
            acc_vsigma = tvsigma0;
            let t132 = t17 / t30 / v_rho;
            let t140 = f64x8::splat(1.0) / t30 / t47;
            let t142 = t27 * t140 * t57;
            let t145 = t44 * t43;
            let t146 = t49 * t49;
            let t148 = f64x8::splat(1.0) / t146 / t29;
            let t153 = f64x8::splat(1.0) / t82 / t56;
            let t155 = t36 * t153 * t20;
            let t156 = t38 * t38;
            let t159 = f64x8::splat(1.0) / t23 / t156 / t38;
            let t160 = t44 * t44;
            let t161 = t160 * t43;
            let t162 = t159 * t161;
            let t163 = t49 * t47;
            let t167 = t27 / t30 / t146 / t163;
            let t171 = t47 * t75;
            let t172 = t49 * t171;
            let t175 = t26 / t18 / t172;
            let t181 = f64x8::splat(1.0) / t95 / t61 * t37;
            let t183 = f64x8::splat(1.0) / t22 / t21;
            let t184 = t181 * t183;
            let t185 = t43 * t26;
            let t187 = f64x8::splat(1.0) / t18 / t171;
            let t194 = f64x8::splat(0.01711111111111111) * t74 * t142 + f64x8::splat(2.245617754729564e-15) * t145 * t148 * t83 - f64x8::splat(2.4334673044738656e-19) * t155 * t162 * t167 - f64x8::splat(1.5278287499791183e-09) * t85 * t86 * t175 - f64x8::splat(0.00962962962962963) * t184 * t185 * t187 + f64x8::splat(0.08881481481481482) * t98 * t28 * t140;
            let t199 = ((t2).select(f64x8::splat(0.0), t6 * t132 * t64 / f64x8::splat(12.0) - t6 * t70 * t102 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t194));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t199 + f64x8::splat(4.0) * t107;
            acc_v2rho2 = tv2rho20;
            let t207 = t146 * v_rho;
            let t209 = f64x8::splat(1.0) / t207 * t83;
            let t212 = t160 * v_sigma;
            let t213 = t159 * t212;
            let t214 = t49 * t75;
            let t218 = t27 / t30 / t146 / t214;
            let t226 = f64x8::splat(1.0) / t18 / t87;
            let t234 = -f64x8::splat(0.004666666666666667) * t25 * t79 - f64x8::splat(8.421066580235865e-16) * t209 * t45 + f64x8::splat(9.125502391776996e-20) * t155 * t213 * t218 + f64x8::splat(5.329635174345761e-10) * t85 * t114 * t91 + f64x8::splat(0.003611111111111111) * t184 * t26 * t226 * v_sigma - f64x8::splat(0.02422222222222222) * t97 * t119 * t77;
            let t239 = ((t2).select(f64x8::splat(0.0), -t6 * t70 * t123 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t234));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t239 + f64x8::splat(2.0) * t127;
            acc_v2rhosigma = tv2rhosigma0;
            let t243 = f64x8::splat(1.0) / t146 * t83;
            let t246 = t159 * t160;
            let t247 = t49 * t29;
            let t251 = t27 / t30 / t146 / t247;
            let t255 = t43 * v_sigma;
            let t256 = t41 * t255;
            let t260 = t183 * t26;
            let t266 = f64x8::splat(3.157899967588449e-16) * t243 * t44 - f64x8::splat(3.4220633969163733e-20) * t155 * t246 * t251 - f64x8::splat(1.5988905523037283e-10) * t85 * t256 * t115 - f64x8::splat(0.0013541666666666667) * t181 * t260 / t18 / t48;
            let t270 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t266));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t270;
            acc_v2sigma2 = tv2sigma20;
            let t273 = t17 * t32;
            let t284 = f64x8::splat(1.0) / t30 / t48;
            let t286 = t27 * t284 * t57;
            let t290 = f64x8::splat(1.0) / t146 / t75;
            let t294 = t160 * t255;
            let t295 = t146 * t146;
            let t297 = f64x8::splat(1.0) / t18 / t295;
            let t300 = t42 * t26;
            let t306 = t37 / t22 / t156 / t39;
            let t307 = t306 * t294;
            let t309 = t26 * t297 * t153;
            let t312 = t82 * t82;
            let t313 = f64x8::splat(1.0) / t312;
            let t314 = t36 * t313;
            let t315 = t44 * t255;
            let t316 = t160 * t315;
            let t317 = t295 * t214;
            let t318 = f64x8::splat(1.0) / t317;
            let t322 = t146 * t50;
            let t325 = t27 / t30 / t322;
            let t331 = t26 / t18 / t146;
            let t335 = t95 * t95;
            let t336 = f64x8::splat(1.0) / t335;
            let t337 = t336 * t255;
            let t338 = f64x8::splat(1.0) / t214;
            let t342 = f64x8::splat(1.0) / t18 / t49;
            let t349 = -f64x8::splat(0.07985185185185185) * t74 * t286 - f64x8::splat(6.063167937769823e-14) * t145 * t290 * t83 - f64x8::splat(8.544420655633364e-25) * t294 * t297 * t153 * t300 - f64x8::splat(4.053700020266563e-21) * t307 * t309 + f64x8::splat(1.9004102083915272e-37) * t314 * t316 * t318 + f64x8::splat(1.0463909409237622e-17) * t155 * t162 * t325 + f64x8::splat(2.3426707499679815e-08) * t85 * t86 * t331 - f64x8::splat(0.00035371070357627984) * t337 * t338 + f64x8::splat(0.10592592592592592) * t184 * t185 * t342 - f64x8::splat(0.41446913580246914) * t98 * t28 * t284;
            let t354 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t273 * t64 + t6 * t132 * t102 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t70 * t194 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t349));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t354 + f64x8::splat(6.0) * t199;
            acc_v3rho3 = tv3rho30;
            let t366 = t148 * t83;
            let t371 = f64x8::splat(1.0) / t18 / t146 / t172;
            let t372 = t371 * t153;
            let t376 = t306 * t161;
            let t378 = t26 * t371 * t153;
            let t381 = t160 * t145;
            let t382 = t295 * t247;
            let t383 = f64x8::splat(1.0) / t382;
            let t394 = t336 / t247;
            let t404 = f64x8::splat(0.01711111111111111) * t25 * t142 + f64x8::splat(2.105266645058966e-14) * t366 * t45 + f64x8::splat(3.204157745862512e-25) * t372 * t161 * t300 + f64x8::splat(1.5201375075999608e-21) * t376 * t378 - f64x8::splat(7.126538281468228e-38) * t314 * t381 * t383 - f64x8::splat(3.741455980628569e-18) * t155 * t213 * t167 - f64x8::splat(7.639143749895592e-09) * t85 * t114 * t175 + f64x8::splat(0.00013264151384110494) * t394 * t43 - f64x8::splat(0.0325) * t184 * t26 * t187 * v_sigma + f64x8::splat(0.08881481481481482) * t97 * t119 * t140;
            let t409 = ((t2).select(f64x8::splat(0.0), t6 * t132 * t123 / f64x8::splat(12.0) - t6 * t70 * t234 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t404));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t409 + f64x8::splat(4.0) * t239;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t417 = t146 * t88;
            let t419 = f64x8::splat(1.0) / t18 / t417;
            let t420 = t419 * t153;
            let t424 = t306 * t212;
            let t429 = t160 * t45;
            let t430 = t49 * v_rho;
            let t432 = f64x8::splat(1.0) / t295 / t430;
            let t443 = t336 / t430;
            let t449 = -f64x8::splat(6.736853264188692e-15) * t209 * t44 - f64x8::splat(1.201559154698442e-25) * t420 * t212 * t300 - f64x8::splat(5.700515653499853e-22) * t424 * t26 * t419 * t153 + f64x8::splat(2.672451855550585e-38) * t314 * t429 * t432 + f64x8::splat(1.2775703348487794e-18) * t155 * t246 * t218 + f64x8::splat(2.1318540697383044e-09) * t85 * t256 * t91 - f64x8::splat(4.974056769041435e-05) * t443 * v_sigma + f64x8::splat(0.007222222222222222) * t181 * t260 * t226;
            let t454 = ((t2).select(f64x8::splat(0.0), -t6 * t70 * t266 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t449));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t454 + f64x8::splat(2.0) * t270;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t459 = f64x8::splat(1.0) / t18 / t322 * t153;
            let t460 = t459 * t160;
            let t465 = t306 * t26;
            let t468 = t160 * t44;
            let t470 = f64x8::splat(1.0) / t295 / t49;
            let t474 = t159 * t315;
            let t478 = t41 * t43;
            let t485 = f64x8::splat(4.505846830119157e-26) * t460 * t300 + f64x8::splat(1.8947399805530697e-15) * t243 * t255 + f64x8::splat(2.137693370062445e-22) * t465 * t460 - f64x8::splat(1.0021694458314695e-38) * t314 * t468 * t470 - f64x8::splat(4.106476076299648e-19) * t155 * t474 * t251 - f64x8::splat(4.796671656911185e-10) * t85 * t478 * t115 + f64x8::splat(1.8652712883905384e-05) * t336 / t49;
            let t489 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t485));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t489;
            acc_v3sigma3 = tv3sigma30;
            let t506 = f64x8::splat(1.0) / t30 / t87;
            let t518 = f64x8::splat(1.0) / t18 / t295 / v_rho;
            let t523 = t160 * t160;
            let t526 = f64x8::splat(1.0) / t30 / t295 / t88;
            let t530 = t20 * t159 * t27;
            let t537 = t156 * t156;
            let t541 = t20 / t23 / t537 / t38;
            let t544 = t27 * t526 * t313;
            let t552 = t36 / t312 / t56;
            let t560 = t41 * t26;
            let t564 = t295 * t163;
            let t582 = f64x8::splat(1.0) / t335 / t61;
            let t587 = t25 * t27;
            let t601 = f64x8::splat(0.4524938271604938) * t74 * t27 * t506 * t57 + f64x8::splat(1.4179828555697902e-12) * t145 / t146 / t47 * t83 + f64x8::splat(5.06968958900913e-23) * t294 * t518 * t153 * t300 + f64x8::splat(2.925983808597274e-33) * t523 * t526 * t313 * t530 + f64x8::splat(3.05378734860081e-19) * t307 * t26 * t518 * t153 + f64x8::splat(1.388164406019777e-29) * t541 * t523 * t544 + f64x8::splat(1.5828634488140606e-39) * t25 * t523 * t544 - f64x8::splat(1.4461859507975415e-46) * t552 * t523 * t44 / t18 / t295 / t146 / t430 * t37 * t560 - f64x8::splat(1.6343527792167136e-35) * t314 * t316 / t564 - f64x8::splat(3.6391151612126397e-16) * t155 * t162 * t27 / t30 / t417 - f64x8::splat(3.826362224947703e-07) * t85 * t86 * t26 / t18 / t207 - f64x8::splat(0.00014061893414347518) * t582 * t44 / t30 / t88 * t587 + f64x8::splat(0.007781635478678156) * t337 / t163 - f64x8::splat(1.0474897119341564) * t184 * t185 / t18 / t430 + f64x8::splat(2.348658436213992) * t98 * t28 * t506;
            let t606 = ((t2).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(27.0) * t6 * t17 * t77 * t64 - f64x8::splat(5.0) / f64x8::splat(9.0) * t6 * t273 * t102 + t6 * t132 * t194 / f64x8::splat(2.0) - t6 * t70 * t349 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t601));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t606 + f64x8::splat(8.0) * t354;
            acc_v4rho4 = tv4rho40;
            let t630 = f64x8::splat(1.0) / t30 / t295 / t50;
            let t639 = t27 * t630 * t313;
            let t680 = -f64x8::splat(0.07985185185185185) * t25 * t286 - f64x8::splat(4.635329315387608e-13) * t290 * t83 * t45 - f64x8::splat(1.8050088635025484e-23) * t297 * t153 * t161 * t300 - f64x8::splat(1.0972439282239778e-33) * t630 * t313 * t316 * t530 - f64x8::splat(1.0995661304973051e-19) * t376 * t309 - f64x8::splat(5.205616522574164e-30) * t541 * t316 * t639 - f64x8::splat(5.9357379330527265e-40) * t25 * t316 * t639 + f64x8::splat(5.42319731549078e-47) * t552 * t523 * t255 / t18 / t295 / t146 / t49 * t37 * t560 + f64x8::splat(5.915026773618629e-36) * t314 * t381 * t318 + f64x8::splat(1.2469492046008166e-16) * t155 * t213 * t325 + f64x8::splat(1.1713353749839907e-07) * t85 * t114 * t331 + f64x8::splat(5.273210030380319e-05) * t582 / t30 / t50 * t255 * t587 - f64x8::splat(0.002520188762980994) * t336 * t338 * t43 + f64x8::splat(0.273641975308642) * t184 * t26 * t342 * v_sigma - f64x8::splat(0.41446913580246914) * t97 * t119 * t284;
            let t685 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t273 * t123 + t6 * t132 * t234 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t70 * t404 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t680));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t685 + f64x8::splat(6.0) * t409;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t701 = f64x8::splat(1.0) / t30 / t564;
            let t710 = t27 * t701 * t313;
            let t746 = f64x8::splat(1.369826830385034e-13) * t366 * t44 + f64x8::splat(6.2080556326086165e-24) * t372 * t212 * t300 + f64x8::splat(4.114664730839917e-34) * t701 * t313 * t381 * t530 + f64x8::splat(3.857348925534901e-20) * t424 * t378 + f64x8::splat(1.9521061959653114e-30) * t541 * t381 * t710 + f64x8::splat(2.2259017248947725e-40) * t25 * t381 * t710 - f64x8::splat(2.0336989933090428e-47) * t552 * t523 * t43 / t18 / t295 / t146 / t171 * t37 * t560 - f64x8::splat(2.093420620181292e-36) * t314 * t429 * t383 - f64x8::splat(4.021304720643063e-17) * t155 * t246 * t167 - f64x8::splat(3.055657499958237e-08) * t85 * t256 * t175 - f64x8::splat(1.9774537613926197e-05) * t582 / t30 / t163 * t43 * t587 + f64x8::splat(0.0007129481368959391) * t394 * v_sigma - f64x8::splat(0.04574074074074074) * t181 * t260 * t187;
            let t751 = ((t2).select(f64x8::splat(0.0), t6 * t132 * t266 / f64x8::splat(12.0) - t6 * t70 * t449 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t746));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t751 + f64x8::splat(4.0) * t454;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t757 = t420 * t160;
            let t761 = f64x8::splat(1.0) / t30 / t317;
            let t763 = t761 * t313 * t429;
            let t770 = t541 * t27;
            let t806 = -f64x8::splat(2.0426505629873512e-24) * t757 * t300 - f64x8::splat(1.5429992740649686e-34) * t763 * t530 - f64x8::splat(3.536847963699063e-14) * t209 * t255 - f64x8::splat(1.3111186003049663e-20) * t465 * t757 - f64x8::splat(7.320398234869918e-31) * t770 * t763 - f64x8::splat(8.347131468355397e-41) * t25 * t429 * t27 * t761 * t313 + f64x8::splat(7.62637122490891e-48) * t552 * t523 * v_sigma / t18 / t295 / t146 / t87 * t37 * t560 + f64x8::splat(7.215620009986581e-37) * t314 * t468 * t432 + f64x8::splat(1.2045663157145635e-17) * t155 * t474 * t218 + f64x8::splat(6.395562209214914e-09) * t85 * t478 * t91 + f64x8::splat(7.415451605222323e-06) * t582 / t30 / t214 * t20 * t24 * v_sigma * t27 - f64x8::splat(0.00014922170307124307) * t443;
            let t811 = ((t2).select(f64x8::splat(0.0), -t6 * t70 * t485 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t806));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t811 + f64x8::splat(2.0) * t489;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t817 = f64x8::splat(1.0) / t30 / t382 * t313 * t468;
            let t820 = t459 * t315;
            let t856 = f64x8::splat(5.786247277743632e-35) * t817 * t530 + f64x8::splat(6.30818556216682e-25) * t820 * t300 + f64x8::splat(7.578959922212279e-15) * t243 * t43 + f64x8::splat(2.745149338076219e-31) * t770 * t817 + f64x8::splat(4.2753867401248904e-21) * t465 * t820 + f64x8::splat(3.130174300633274e-41) * t587 * t817 - f64x8::splat(2.859889209340841e-48) * t552 * t523 / t18 / t295 / t146 / t48 * t37 * t560 - f64x8::splat(2.4052066699955267e-37) * t314 * t294 * t470 - f64x8::splat(3.2851808610397184e-18) * t155 * t159 * t145 * t251 - f64x8::splat(9.59334331382237e-10) * t85 * t41 * v_sigma * t115 - f64x8::splat(2.7807943519583713e-06) * t582 / t30 / t247 * t587;
            let t860 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t856));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t860;
            acc_v4sigma4 = tv4sigma40;
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
        { let a: [f64; 8] = acc_v4rho4.into(); v4rho4[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho3sigma.into(); v4rho3sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho2sigma2.into(); v4rho2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rhosigma3.into(); v4rhosigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4sigma4.into(); v4sigma4[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
