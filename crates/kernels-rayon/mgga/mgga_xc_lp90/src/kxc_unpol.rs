//! MGGA_XC_LP90 kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_lp90.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_xc_lp90_kxc_unpol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] * rho[ip];
        let t3 = pow_1_3(rho[ip]);
        let t4 = t3 * t3;
        let t6 = 1.0 / t4 / t2;
        let t10 = 1.0 / t4 / rho[ip];
        let t13 = 0.80569 + 0.00037655 * sigma[ip] * t6 - 0.00037655 * lapl[ip] * t10;
        let t14 = 1.0 / t3;
        let t15 = t14 + 0.0040743;
        let t16 = 1.0 / t15;
        let tzk0 = -t13 * t16;
        zk[ip] += tzk0;
        let t18 = t2 * rho[ip];
        let t20 = 1.0 / t4 / t18;
        let t25 = -0.0010041333333333333 * sigma[ip] * t20 + 0.0006275833333333333 * lapl[ip] * t6;
        let t29 = t15 * t15;
        let t30 = 1.0 / t29;
        let tvrho0 = tzk0 - rho[ip] * t25 * t16 - t14 * t13 * t30 / 3.0;
        vrho[ip] += tvrho0;
        let t33 = t10 * t16;
        let tvsigma0 = -0.00037655 * t33;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.00037655 / t4 * t16;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
        let t39 = t13 * t30;
        let t41 = 1.0 / t3 / rho[ip];
        let t44 = t2 * t2;
        let t46 = 1.0 / t4 / t44;
        let t51 = 0.0036818222222222224 * sigma[ip] * t46 - 0.0016735555555555555 * lapl[ip] * t20;
        let t59 = 1.0 / t29 / t15;
        let tv2rho20 = -2.0 * t25 * t16 - 2.0 / 9.0 * t39 * t41 - rho[ip] * t51 * t16 - 2.0 / 3.0 * t14 * t25 * t30 - 2.0 / 9.0 * t10 * t13 * t59;
        v2rho2[ip] += tv2rho20;
        let t62 = t6 * t16;
        let t64 = 1.0 / t18;
        let t65 = t64 * t30;
        let tv2rhosigma0 = 0.0006275833333333333 * t62 - 0.00012551666666666666 * t65;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = -0.00025103333333333333 * t33 + 0.00012551666666666666 / t2 * t30;
        v2rholapl[ip] += tv2rholapl0;
        let tv2rhotau0 = 0.0;
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
        let t73 = t25 * t30;
        let t76 = t13 * t59;
        let t80 = 1.0 / t3 / t2;
        let t83 = t44 * rho[ip];
        let t85 = 1.0 / t4 / t83;
        let t90 = -0.01718183703703704 * sigma[ip] * t85 + 0.00613637037037037 * lapl[ip] * t46;
        let t99 = t29 * t29;
        let t100 = 1.0 / t99;
        let tv3rho30 = -3.0 * t51 * t16 - 2.0 / 3.0 * t73 * t41 + 2.0 / 9.0 * t76 * t6 + 8.0 / 27.0 * t39 * t80 - rho[ip] * t90 * t16 - t14 * t51 * t30 - 2.0 / 3.0 * t10 * t25 * t59 - 2.0 / 9.0 * t64 * t13 * t100;
        v3rho3[ip] += tv3rho30;
        let t103 = t20 * t16;
        let t105 = 1.0 / t44;
        let t106 = t105 * t30;
        let t109 = 1.0 / t3 / t44;
        let t110 = t109 * t59;
        let tv3rho2sigma0 = -0.0016735555555555555 * t103 + 0.0005857444444444445 * t106 - 8.367777777777778e-05 * t110;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t115 = 1.0 / t3 / t18;
        let tv3rho2lapl0 = 0.0004183888888888889 * t62 - 0.0003347111111111111 * t65 + 8.367777777777778e-05 * t115 * t59;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let tv3rho2tau0 = 0.0;
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
