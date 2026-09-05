//! GGA_XC_TH2 lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_th2.c`
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
pub fn gga_xc_th2_lxc_unpol(
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
            let t1 = (simd::pow(f64x8::splat(2.0), f64x8::splat(1.0) / f64x8::splat(12.0)));
            let t2 = t1 * t1;
            let t3 = t2 * t1;
            let t4 = t2 * t2;
            let t5 = t4 * t4;
            let t6 = t5 * t3;
            let t7 = (simd::pow(v_rho, f64x8::splat(1.0) / f64x8::splat(12.0)));
            let t11 = (simd::pow(f64x8::splat(2.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t12 = t11 * t11;
            let t13 = t12 * t12;
            let t14 = t13 * t11;
            let t15 = (simd::pow(v_rho, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t16 = t15 * v_rho;
            let t19 = f64x8::splat(M_CBRT2);
            let t20 = t19 * t19;
            let t21 = (simd::cbrt(v_rho));
            let t22 = t21 * v_rho;
            let t25 = f64x8::splat(M_SQRT2);
            let t26 = ((v_rho).sqrt());
            let t27 = t26 * v_rho;
            let t30 = t21 * t21;
            let t31 = t30 * v_rho;
            let t32 = t19 * t31;
            let t34 = t4 * t3;
            let t35 = t34 * t7;
            let t36 = ((v_sigma).sqrt());
            let t38 = (simd::cbrt(zeta_threshold));
            let t40 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t38 * zeta_threshold, f64x8::splat(1.0)));
            let t41 = t36 * t40;
            let t44 = t25 * t15;
            let t47 = t19 * t21;
            let t50 = t11 * t26;
            let t53 = f64x8::splat(1.0) / v_rho;
            let t54 = t19 * t53;
            let t55 = t40 * t40;
            let t56 = v_sigma * t55;
            let t59 = t15 * t15;
            let t60 = t59 * t59;
            let t61 = t60 * t15;
            let t62 = f64x8::splat(1.0) / t61;
            let t63 = t11 * t62;
            let t66 = f64x8::splat(1.0) / t30;
            let t70 = v_rho * v_rho;
            let t72 = f64x8::splat(1.0) / t30 / t70;
            let t73 = v_sigma * t72;
            let t74 = t73 * t55;
            let t75 = t74 - t73;
            let t78 = t61 * v_rho;
            let t79 = t11 * t78;
            let t84 = f64x8::splat(0.3394155) * t6 * t7 * v_rho - f64x8::splat(0.879105) * t14 * t16 + f64x8::splat(0.63838) * t20 * t22 - f64x8::splat(0.803945) * t25 * t27 + f64x8::splat(0.182805) * t32 - f64x8::splat(0.04533175) * t35 * t41 + f64x8::splat(0.03674325) * t44 * t41 + f64x8::splat(0.03678525) * t47 * t41 - f64x8::splat(0.017922925) * t50 * t41 - f64x8::splat(0.0050895875) * t54 * t56 + f64x8::splat(0.0026828125) * t63 * t56 - f64x8::splat(9.60195e-05) * t66 * v_sigma * t55 + f64x8::splat(0.01551885) * t32 * t75 - f64x8::splat(0.0360163) * t79 * t75 + f64x8::splat(0.0223281) * t70 * t75;
            let tzk0 = t84 * t53;
            acc_zk = tzk0;
            let t93 = t19 * t30;
            let t95 = t7 * t7;
            let t97 = t95 * t95;
            let t98 = t97 * t97;
            let t99 = t98 * t95 * t7;
            let t100 = f64x8::splat(1.0) / t99;
            let t101 = t34 * t100;
            let t104 = t25 * t62;
            let t107 = t19 * t66;
            let t110 = f64x8::splat(1.0) / t26;
            let t111 = t11 * t110;
            let t115 = t19 / t70;
            let t118 = f64x8::splat(1.0) / t78;
            let t119 = t11 * t118;
            let t122 = f64x8::splat(1.0) / t31;
            let t128 = t70 * v_rho;
            let t130 = f64x8::splat(1.0) / t30 / t128;
            let t131 = v_sigma * t130;
            let t132 = t131 * t55;
            let t134 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t132 + f64x8::splat(8.0) / f64x8::splat(3.0) * t131;
            let t137 = t11 * t61;
            let tvrho0 = f64x8::splat(0.367700125) * t6 * t7 - f64x8::splat(1.0256225) * t14 * t15 + f64x8::splat(0.8511733333333333) * t20 * t21 - f64x8::splat(1.2059175) * t25 * t26 + f64x8::splat(0.304675) * t93 - f64x8::splat(0.0037776458333333334) * t101 * t41 + f64x8::splat(0.006123875) * t104 * t41 + f64x8::splat(0.01226175) * t107 * t41 - f64x8::splat(0.0089614625) * t111 * t41 + f64x8::splat(0.0050895875) * t115 * t56 - f64x8::splat(0.0022356770833333334) * t119 * t56 + f64x8::splat(6.4013e-05) * t122 * v_sigma * t55 + f64x8::splat(0.02586475) * t93 * t75 + f64x8::splat(0.01551885) * t32 * t134 - f64x8::splat(0.06602988333333333) * t137 * t75 - f64x8::splat(0.0360163) * t79 * t134 + f64x8::splat(0.0446562) * v_rho * t75 + f64x8::splat(0.0223281) * t70 * t134;
            acc_vrho = tvrho0;
            let t147 = f64x8::splat(1.0) / t36 * t40;
            let t162 = t72 * t55;
            let t163 = t162 - t72;
            let tvsigma0 = -f64x8::splat(0.022665875) * t35 * t147 + f64x8::splat(0.018371625) * t44 * t147 + f64x8::splat(0.018392625) * t47 * t147 - f64x8::splat(0.0089614625) * t50 * t147 - f64x8::splat(0.0050895875) * t54 * t55 + f64x8::splat(0.0026828125) * t63 * t55 - f64x8::splat(9.60195e-05) * t66 * t55 + f64x8::splat(0.01551885) * t32 * t163 - f64x8::splat(0.0360163) * t79 * t163 + f64x8::splat(0.0223281) * t70 * t163;
            acc_vsigma = tvsigma0;
            let t171 = f64x8::splat(1.0) / t99 / v_rho;
            let t172 = t34 * t171;
            let t175 = t25 * t118;
            let t178 = t19 * t122;
            let t181 = f64x8::splat(1.0) / t27;
            let t182 = t11 * t181;
            let t186 = t19 / t128;
            let t190 = f64x8::splat(1.0) / t61 / t70;
            let t191 = t11 * t190;
            let t195 = t19 / t21;
            let t199 = t11 / t15;
            let t207 = t70 * t70;
            let t209 = f64x8::splat(1.0) / t30 / t207;
            let t210 = v_sigma * t209;
            let t211 = t210 * t55;
            let t213 = f64x8::splat(88.0) / f64x8::splat(9.0) * t211 - f64x8::splat(88.0) / f64x8::splat(9.0) * t210;
            let t233 = f64x8::splat(0.0517295) * t93 * t134 + f64x8::splat(0.01551885) * t32 * t213 - f64x8::splat(0.13205976666666666) * t137 * t134 - f64x8::splat(0.0360163) * t79 * t213 + f64x8::splat(0.030641677083333332) * t6 * t100 - f64x8::splat(0.17093708333333332) * t14 * t62 + f64x8::splat(0.28372444444444445) * t20 * t66 - f64x8::splat(0.60295875) * t25 * t110 + f64x8::splat(0.20311666666666667) * t195 + f64x8::splat(0.0893124) * v_rho * t134 + f64x8::splat(0.0223281) * t70 * t213;
            let tv2rho20 = f64x8::splat(0.003462842013888889) * t172 * t41 - f64x8::splat(0.0051032291666666665) * t175 * t41 - f64x8::splat(0.0081745) * t178 * t41 + f64x8::splat(0.00448073125) * t182 * t41 - f64x8::splat(0.010179175) * t186 * t56 + f64x8::splat(0.004098741319444444) * t191 * t56 + f64x8::splat(0.017243166666666667) * t195 * t75 - f64x8::splat(0.05502490277777778) * t199 * t75 + f64x8::splat(0.044549511666666666) * t74 - f64x8::splat(0.0446562) * t73 + t233;
            acc_v2rho2 = tv2rho20;
            let t250 = t130 * t55;
            let t252 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t250 + f64x8::splat(8.0) / f64x8::splat(3.0) * t130;
            let tv2rhosigma0 = -f64x8::splat(0.0018888229166666667) * t101 * t147 + f64x8::splat(0.0030619375) * t104 * t147 + f64x8::splat(0.006130875) * t107 * t147 - f64x8::splat(0.00448073125) * t111 * t147 + f64x8::splat(0.0050895875) * t115 * t55 - f64x8::splat(0.0022356770833333334) * t119 * t55 + f64x8::splat(6.4013e-05) * t122 * t55 + f64x8::splat(0.02586475) * t93 * t163 + f64x8::splat(0.01551885) * t32 * t252 - f64x8::splat(0.06602988333333333) * t137 * t163 - f64x8::splat(0.0360163) * t79 * t252 + f64x8::splat(0.0446562) * v_rho * t163 + f64x8::splat(0.0223281) * t70 * t252;
            acc_v2rhosigma = tv2rhosigma0;
            let t265 = f64x8::splat(1.0) / t36 / v_sigma * t40;
            let tv2sigma20 = f64x8::splat(0.0113329375) * t35 * t265 - f64x8::splat(0.0091858125) * t44 * t265 - f64x8::splat(0.0091963125) * t47 * t265 + f64x8::splat(0.00448073125) * t50 * t265;
            acc_v2sigma2 = tv2sigma20;
            let t283 = t19 / t22;
            let t287 = t207 * v_rho;
            let t289 = f64x8::splat(1.0) / t30 / t287;
            let t290 = v_sigma * t289;
            let t293 = -f64x8::splat(1232.0) / f64x8::splat(27.0) * t290 * t55 + f64x8::splat(1232.0) / f64x8::splat(27.0) * t290;
            let t304 = -f64x8::splat(0.028088203993055557) * t6 * t171 + f64x8::splat(0.14244756944444445) * t14 * t118 - f64x8::splat(0.18914962962962964) * t20 * t122 + f64x8::splat(0.301479375) * t25 * t181 - f64x8::splat(0.06770555555555556) * t283 + f64x8::splat(0.1339686) * v_rho * t213 + f64x8::splat(0.0223281) * t70 * t293 + f64x8::splat(0.0517295) * t195 * t134 - f64x8::splat(0.16507470833333332) * t199 * t134 + f64x8::splat(0.07759425) * t93 * t213 + f64x8::splat(0.01551885) * t32 * t293;
            let t310 = f64x8::splat(1.0) / t99 / t70;
            let t311 = t34 * t310;
            let t314 = t25 * t190;
            let t317 = t19 * t72;
            let t321 = f64x8::splat(1.0) / t26 / t70;
            let t322 = t11 * t321;
            let t326 = t19 / t207;
            let t330 = f64x8::splat(1.0) / t61 / t128;
            let t331 = t11 * t330;
            let t337 = t11 / t16;
            let t342 = -f64x8::splat(0.19808965) * t137 * t213 - f64x8::splat(0.0360163) * t79 * t293 - f64x8::splat(0.006637113859953704) * t311 * t41 + f64x8::splat(0.009355920138888888) * t314 * t41 + f64x8::splat(0.013624166666666666) * t317 * t41 - f64x8::splat(0.006721096875) * t322 * t41 + f64x8::splat(0.030537525) * t326 * t56 - f64x8::splat(0.011613100405092593) * t331 * t56 - f64x8::splat(0.005747722222222223) * t283 * t75 + f64x8::splat(0.00917081712962963) * t337 * t75 + f64x8::splat(0.3572496) * t131 - f64x8::splat(0.3569650977777778) * t132;
            let tv3rho30 = t304 + t342;
            acc_v3rho3 = tv3rho30;
            let t362 = f64x8::splat(88.0) / f64x8::splat(9.0) * t209 * t55 - f64x8::splat(88.0) / f64x8::splat(9.0) * t209;
            let tv3rho2sigma0 = f64x8::splat(0.0017314210069444445) * t172 * t147 - f64x8::splat(0.0025516145833333333) * t175 * t147 - f64x8::splat(0.00408725) * t178 * t147 + f64x8::splat(0.002240365625) * t182 * t147 - f64x8::splat(0.010179175) * t186 * t55 + f64x8::splat(0.004098741319444444) * t191 * t55 + f64x8::splat(0.044549511666666666) * t162 + f64x8::splat(0.017243166666666667) * t195 * t163 + f64x8::splat(0.0517295) * t93 * t252 + f64x8::splat(0.01551885) * t32 * t362 - f64x8::splat(0.05502490277777778) * t199 * t163 - f64x8::splat(0.13205976666666666) * t137 * t252 - f64x8::splat(0.0360163) * t79 * t362 - f64x8::splat(0.0446562) * t72 + f64x8::splat(0.0893124) * v_rho * t252 + f64x8::splat(0.0223281) * t70 * t362;
            acc_v3rho2sigma = tv3rho2sigma0;
            let tv3rhosigma20 = f64x8::splat(0.0009444114583333333) * t101 * t265 - f64x8::splat(0.00153096875) * t104 * t265 - f64x8::splat(0.0030654375) * t107 * t265 + f64x8::splat(0.002240365625) * t111 * t265;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t384 = v_sigma * v_sigma;
            let t387 = f64x8::splat(1.0) / t36 / t384 * t40;
            let tv3sigma30 = -f64x8::splat(0.01699940625) * t35 * t387 + f64x8::splat(0.01377871875) * t44 * t387 + f64x8::splat(0.01379446875) * t47 * t387 - f64x8::splat(0.006721096875) * t50 * t387;
            acc_v3sigma3 = tv3sigma30;
            let t404 = t19 / t21 / t70;
            let t411 = v_sigma / t30 / t207 / t70;
            let t414 = f64x8::splat(20944.0) / f64x8::splat(81.0) * t411 * t55 - f64x8::splat(20944.0) / f64x8::splat(81.0) * t411;
            let t428 = -f64x8::splat(0.2611538773148148) * t14 * t190 + f64x8::splat(0.3152493827160494) * t20 * t72 - f64x8::splat(0.4522190625) * t25 * t321 + f64x8::splat(0.09027407407407408) * t404 + f64x8::splat(0.1786248) * v_rho * t293 + f64x8::splat(0.0223281) * t70 * t414 + f64x8::splat(0.05383572432002315) * t6 * t310 - f64x8::splat(2.6198304) * t210 + f64x8::splat(0.103459) * t195 * t213 - f64x8::splat(0.33014941666666664) * t199 * t213 + f64x8::splat(0.103459) * t93 * t293 + f64x8::splat(0.01551885) * t32 * t414;
            let t470 = -f64x8::splat(0.2641195333333333) * t137 * t293 - f64x8::splat(0.0360163) * t79 * t414 - f64x8::splat(0.02299088888888889) * t283 * t134 + f64x8::splat(0.03668326851851852) * t337 * t134 + f64x8::splat(0.00766362962962963) * t404 * t75 - f64x8::splat(0.010699286651234569) * t11 / t15 / t70 * t75 + f64x8::splat(2.6187872251851854) * t211 + f64x8::splat(0.0193582487581983) * t34 / t99 / t128 * t41 - f64x8::splat(0.026508440393518518) * t25 * t330 * t41 - f64x8::splat(0.03633111111111111) * t19 * t130 * t41 + f64x8::splat(0.0168027421875) * t11 / t26 / t128 * t41 - f64x8::splat(0.1221501) * t19 / t287 * t56 + f64x8::splat(0.04451688488618827) * t11 / t61 / t207 * t56;
            let tv4rho40 = t428 + t470;
            acc_v4rho4 = tv4rho40;
            let t475 = -f64x8::splat(1232.0) / f64x8::splat(27.0) * t289 * t55 + f64x8::splat(1232.0) / f64x8::splat(27.0) * t289;
            let tv4rho3sigma0 = f64x8::splat(0.1339686) * v_rho * t362 + f64x8::splat(0.0223281) * t70 * t475 + f64x8::splat(0.030537525) * t326 * t55 - f64x8::splat(0.011613100405092593) * t331 * t55 + f64x8::splat(0.0517295) * t195 * t252 + f64x8::splat(0.07759425) * t93 * t362 + f64x8::splat(0.01551885) * t32 * t475 - f64x8::splat(0.16507470833333332) * t199 * t252 - f64x8::splat(0.19808965) * t137 * t362 - f64x8::splat(0.0360163) * t79 * t475 - f64x8::splat(0.003318556929976852) * t311 * t147 + f64x8::splat(0.004677960069444444) * t314 * t147 + f64x8::splat(0.006812083333333333) * t317 * t147 - f64x8::splat(0.0033605484375) * t322 * t147 - f64x8::splat(0.005747722222222223) * t283 * t163 + f64x8::splat(0.00917081712962963) * t337 * t163 + f64x8::splat(0.3572496) * t130 - f64x8::splat(0.3569650977777778) * t250;
            acc_v4rho3sigma = tv4rho3sigma0;
            let tv4rho2sigma20 = -f64x8::splat(0.0008657105034722223) * t172 * t265 + f64x8::splat(0.0012758072916666666) * t175 * t265 + f64x8::splat(0.002043625) * t178 * t265 - f64x8::splat(0.0011201828125) * t182 * t265;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let tv4rhosigma30 = -f64x8::splat(0.0014166171875) * t101 * t387 + f64x8::splat(0.002296453125) * t104 * t387 + f64x8::splat(0.00459815625) * t107 * t387 - f64x8::splat(0.0033605484375) * t111 * t387;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t527 = f64x8::splat(1.0) / t36 / t384 / v_sigma * t40;
            let tv4sigma40 = f64x8::splat(0.042498515625) * t35 * t527 - f64x8::splat(0.034446796875) * t44 * t527 - f64x8::splat(0.034486171875) * t47 * t527 + f64x8::splat(0.0168027421875) * t50 * t527;
            acc_v4sigma4 = tv4sigma40;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v3rho3, ip, m, acc_v3rho3);
        store_add(v3rho2sigma, ip, m, acc_v3rho2sigma);
        store_add(v3rhosigma2, ip, m, acc_v3rhosigma2);
        store_add(v3sigma3, ip, m, acc_v3sigma3);
        store_add(v4rho4, ip, m, acc_v4rho4);
        store_add(v4rho3sigma, ip, m, acc_v4rho3sigma);
        store_add(v4rho2sigma2, ip, m, acc_v4rho2sigma2);
        store_add(v4rhosigma3, ip, m, acc_v4rhosigma3);
        store_add(v4sigma4, ip, m, acc_v4sigma4);
        ip += 8;
    }
}
