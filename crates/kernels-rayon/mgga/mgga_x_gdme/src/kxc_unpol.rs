//! MGGA_X_GDME kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gdme.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_gdme_kxc_unpol(
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
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rho2lapl: &mut [f64],
    v3rho2tau: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3rhosigmalapl: &mut [f64],
    v3rhosigmatau: &mut [f64],
    v3rholapl2: &mut [f64],
    v3rholapltau: &mut [f64],
    v3rhotau2: &mut [f64],
    v3sigma3: &mut [f64],
    v3sigma2lapl: &mut [f64],
    v3sigma2tau: &mut [f64],
    v3sigmalapl2: &mut [f64],
    v3sigmalapltau: &mut [f64],
    v3sigmatau2: &mut [f64],
    v3lapl3: &mut [f64],
    v3lapl2tau: &mut [f64],
    v3lapltau2: &mut [f64],
    v3tau3: &mut [f64],
    param_AA: f64,
    param_BB: f64,
    param_a: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t6 = 1.0 / t5;
        let t7 = t4 * t6;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t23 = M_CBRT2;
        let t26 = pow_1_3(1.0 / M_PI);
        let t27 = 1.0 / t26;
        let t28 = M_CBRT4;
        let t29 = t27 * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t39 = t23 * t23;
        let t42 = 1.0 / t31 / t30;
        let t43 = param_a * param_a;
        let t44 = t43 - param_a + 1.0 / 2.0;
        let t45 = t44 * lapl[ip];
        let t46 = t19 * t19;
        let t48 = 1.0 / t46 / rho[ip];
        let t51 = tau[ip] * t39;
        let t59 = 2.0 / 9.0 * (param_AA + 3.0 / 5.0 * param_BB) * t23 * t29 / t32 + param_BB * t4 * t27 * t28 * t39 * t42 * (t45 * t39 * t48 - 2.0 * t51 * t48) / 27.0;
        let t63 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t59);
        let tzk0 = 2.0 * t63;
        zk[ip] += tzk0;
        let t65 = t18 / t46;
        let t69 = t4 * t4;
        let t70 = t69 * t6;
        let t72 = t70 * t20 * param_BB;
        let t73 = t39 * t42;
        let t74 = rho[ip] * rho[ip];
        let t76 = 1.0 / t46 / t74;
        let t84 = t29 * t73 * (-5.0 / 3.0 * t45 * t39 * t76 + 10.0 / 3.0 * t51 * t76);
        let t88 = piecewise3(t3, 0.0, -t7 * t65 * t59 / 8.0 - t72 * t84 / 72.0);
        let tvrho0 = 2.0 * rho[ip] * t88 + 2.0 * t63;
        vrho[ip] += tvrho0;
        let tvsigma0 = 0.0;
        vsigma[ip] += tvsigma0;
        let t93 = t18 / t19 / rho[ip];
        let t98 = t29 * t23 * t42 * t44;
        let t101 = piecewise3(t3, 0.0, -t70 * t93 * param_BB * t98 / 36.0);
        let tvlapl0 = 2.0 * rho[ip] * t101;
        vlapl[ip] += tvlapl0;
        let t107 = param_BB * t27 * t28 * t23 * t42;
        let t110 = piecewise3(t3, 0.0, t70 * t93 * t107 / 18.0);
        let tvtau0 = 2.0 * rho[ip] * t110;
        vtau[ip] += tvtau0;
        let t113 = t18 * t48;
        let t118 = t70 * t65 * param_BB;
        let t121 = t74 * rho[ip];
        let t123 = 1.0 / t46 / t121;
        let t131 = t29 * t73 * (40.0 / 9.0 * t45 * t39 * t123 - 80.0 / 9.0 * t51 * t123);
        let t135 = piecewise3(t3, 0.0, t7 * t113 * t59 / 12.0 - t118 * t84 / 108.0 - t72 * t131 / 72.0);
        let tv2rho20 = 2.0 * rho[ip] * t135 + 4.0 * t88;
        v2rho2[ip] += tv2rho20;
        let tv2rhosigma0 = 0.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let t140 = t18 / t19 / t74;
        let t145 = piecewise3(t3, 0.0, t70 * t140 * param_BB * t98 / 27.0);
        let tv2rholapl0 = 2.0 * rho[ip] * t145 + 2.0 * t101;
        v2rholapl[ip] += tv2rholapl0;
        let t151 = piecewise3(t3, 0.0, -2.0 / 27.0 * t70 * t140 * t107);
        let tv2rhotau0 = 2.0 * rho[ip] * t151 + 2.0 * t110;
        v2rhotau[ip] += tv2rhotau0;
        let tv2sigma20 = 0.0;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let tv2tau20 = 0.0;
        v2tau2[ip] += tv2tau20;
        let t155 = t18 * t76;
        let t160 = t70 * t113 * param_BB;
        let t165 = t74 * t74;
        let t167 = 1.0 / t46 / t165;
        let t175 = t29 * t73 * (-440.0 / 27.0 * t45 * t39 * t167 + 880.0 / 27.0 * t51 * t167);
        let t179 = piecewise3(t3, 0.0, -5.0 / 36.0 * t7 * t155 * t59 + t160 * t84 / 108.0 - t118 * t131 / 72.0 - t72 * t175 / 72.0);
        let tv3rho30 = 2.0 * rho[ip] * t179 + 6.0 * t135;
        v3rho3[ip] += tv3rho30;
        let tv3rho2sigma0 = 0.0;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t185 = t18 / t19 / t121;
        let t190 = piecewise3(t3, 0.0, -7.0 / 81.0 * t70 * t185 * param_BB * t98);
        let tv3rho2lapl0 = 2.0 * rho[ip] * t190 + 4.0 * t145;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let t197 = piecewise3(t3, 0.0, 14.0 / 81.0 * t70 * t185 * t107);
        let tv3rho2tau0 = 2.0 * rho[ip] * t197 + 4.0 * t151;
        v3rho2tau[ip] += tv3rho2tau0;
        let tv3rhosigma20 = 0.0;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3rhosigmalapl0 = 0.0;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let tv3rhosigmatau0 = 0.0;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let tv3rholapl20 = 0.0;
        v3rholapl2[ip] += tv3rholapl20;
        let tv3rholapltau0 = 0.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let tv3rhotau20 = 0.0;
        v3rhotau2[ip] += tv3rhotau20;
        let tv3sigma30 = 0.0;
        v3sigma3[ip] += tv3sigma30;
        let tv3sigma2lapl0 = 0.0;
        v3sigma2lapl[ip] += tv3sigma2lapl0;
        let tv3sigma2tau0 = 0.0;
        v3sigma2tau[ip] += tv3sigma2tau0;
        let tv3sigmalapl20 = 0.0;
        v3sigmalapl2[ip] += tv3sigmalapl20;
        let tv3sigmalapltau0 = 0.0;
        v3sigmalapltau[ip] += tv3sigmalapltau0;
        let tv3sigmatau20 = 0.0;
        v3sigmatau2[ip] += tv3sigmatau20;
        let tv3lapl30 = 0.0;
        v3lapl3[ip] += tv3lapl30;
        let tv3lapl2tau0 = 0.0;
        v3lapl2tau[ip] += tv3lapl2tau0;
        let tv3lapltau20 = 0.0;
        v3lapltau2[ip] += tv3lapltau20;
        let tv3tau30 = 0.0;
        v3tau3[ip] += tv3tau30;
    }
}
