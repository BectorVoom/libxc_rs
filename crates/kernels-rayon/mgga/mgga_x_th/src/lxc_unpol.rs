//! MGGA_X_TH lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_th.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_th_lxc_unpol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho3lapl: &mut [f64],
    v4rho3tau: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rho2sigmalapl: &mut [f64],
    v4rho2sigmatau: &mut [f64],
    v4rho2lapl2: &mut [f64],
    v4rho2lapltau: &mut [f64],
    v4rho2tau2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4rhosigma2lapl: &mut [f64],
    v4rhosigma2tau: &mut [f64],
    v4rhosigmalapl2: &mut [f64],
    v4rhosigmalapltau: &mut [f64],
    v4rhosigmatau2: &mut [f64],
    v4rholapl3: &mut [f64],
    v4rholapl2tau: &mut [f64],
    v4rholapltau2: &mut [f64],
    v4rhotau3: &mut [f64],
    v4sigma4: &mut [f64],
    v4sigma3lapl: &mut [f64],
    v4sigma3tau: &mut [f64],
    v4sigma2lapl2: &mut [f64],
    v4sigma2lapltau: &mut [f64],
    v4sigma2tau2: &mut [f64],
    v4sigmalapl3: &mut [f64],
    v4sigmalapl2tau: &mut [f64],
    v4sigmalapltau2: &mut [f64],
    v4sigmatau3: &mut [f64],
    v4lapl4: &mut [f64],
    v4lapl3tau: &mut [f64],
    v4lapl2tau2: &mut [f64],
    v4lapltau3: &mut [f64],
    v4tau4: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRTPI;
        let t5 = t4 * t4;
        let t6 = 1.0 <= zeta_threshold;
        let t7 = zeta_threshold - 1.0;
        let t9 = piecewise5(t6, t7, t6, -t7, 0.0);
        let t10 = 1.0 + t9;
        let t12 = pow_1_3(zeta_threshold);
        let t14 = pow_1_3(t10);
        let t16 = piecewise3(t10 <= zeta_threshold, t12 * zeta_threshold, t14 * t10);
        let t17 = t5 * t16;
        let t18 = rho[ip] * rho[ip];
        let t19 = 1.0 / tau[ip];
        let t22 = M_CBRT2;
        let t23 = 1.0 / rho[ip];
        let t30 = pow_1_3(1.0 / M_PI);
        let t31 = 1.0 / t30;
        let t32 = M_CBRT4;
        let t33 = t31 * t32;
        let t34 = t22 * (1.0 + 7.0 / 216.0 * sigma[ip] * t23 * t19) * t33;
        let t37 = piecewise3(t3, 0.0, -27.0 / 160.0 * t17 * t18 * t19 * t34);
        let tzk0 = 2.0 * t37;
        zk[ip] += tzk0;
        let t42 = tau[ip] * tau[ip];
        let t43 = 1.0 / t42;
        let t44 = t17 * t43;
        let t46 = t22 * sigma[ip] * t33;
        let t50 = piecewise3(t3, 0.0, -27.0 / 80.0 * t17 * rho[ip] * t19 * t34 + 7.0 / 1280.0 * t44 * t46);
        let tvrho0 = 2.0 * rho[ip] * t50 + 2.0 * t37;
        vrho[ip] += tvrho0;
        let t53 = t17 * rho[ip];
        let t58 = piecewise3(t3, 0.0, -7.0 / 1280.0 * t53 * t43 * t22 * t33);
        let tvsigma0 = 2.0 * rho[ip] * t58;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t65 = 1.0 / t42 / tau[ip];
        let t67 = t17 * rho[ip] * t65;
        let t71 = piecewise3(t3, 0.0, 27.0 / 160.0 * t17 * t18 * t43 * t34 + 7.0 / 1280.0 * t67 * t46);
        let tvtau0 = 2.0 * rho[ip] * t71;
        vtau[ip] += tvtau0;
        let t82 = piecewise3(t3, 0.0, -27.0 / 80.0 * t17 * t19 * t34 + 7.0 / 640.0 * t17 * t23 * t43 * t46);
        let tv2rho20 = 2.0 * rho[ip] * t82 + 4.0 * t50;
        v2rho2[ip] += tv2rho20;
        let t86 = t22 * t31 * t32;
        let t89 = piecewise3(t3, 0.0, -7.0 / 1280.0 * t44 * t86);
        let tv2rhosigma0 = 2.0 * rho[ip] * t89 + 2.0 * t58;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t96 = piecewise3(t3, 0.0, 27.0 / 80.0 * t17 * rho[ip] * t43 * t34);
        let tv2rhotau0 = 2.0 * rho[ip] * t96 + 2.0 * t71;
        v2rhotau[ip] += tv2rhotau0;
        let tv2sigma20 = 0.0;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t103 = piecewise3(t3, 0.0, 7.0 / 640.0 * t53 * t65 * t22 * t33);
        let tv2sigmatau0 = 2.0 * rho[ip] * t103;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t109 = t42 * t42;
        let t110 = 1.0 / t109;
        let t112 = t17 * rho[ip] * t110;
        let t116 = piecewise3(t3, 0.0, -27.0 / 80.0 * t17 * t18 * t65 * t34 - 7.0 / 320.0 * t112 * t46);
        let tv2tau20 = 2.0 * rho[ip] * t116;
        v2tau2[ip] += tv2tau20;
        let t119 = piecewise3(t3, 0.0, 0.0);
        let t121 = 2.0 * rho[ip] * t119;
        let tv3rho30 = 6.0 * t82 + t121;
        v3rho3[ip] += tv3rho30;
        let tv3rho2sigma0 = 4.0 * t89 + t121;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let tv3rho2lapl0 = 0.0;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let t131 = piecewise3(t3, 0.0, 27.0 / 80.0 * t44 * t34 - 7.0 / 640.0 * t17 * t23 * t65 * t46);
        let tv3rho2tau0 = 2.0 * rho[ip] * t131 + 4.0 * t96;
        v3rho2tau[ip] += tv3rho2tau0;
        let tv3rhosigma20 = 0.0;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3rhosigmalapl0 = 0.0;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let t134 = t17 * t65;
        let t137 = piecewise3(t3, 0.0, 7.0 / 640.0 * t134 * t86);
        let tv3rhosigmatau0 = 2.0 * rho[ip] * t137 + 2.0 * t103;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let tv3rholapl20 = 0.0;
        v3rholapl2[ip] += tv3rholapl20;
        let tv3rholapltau0 = 0.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let t142 = t17 * t110;
        let t146 = piecewise3(t3, 0.0, -27.0 / 40.0 * t67 * t34 - 7.0 / 640.0 * t142 * t46);
        let tv3rhotau20 = 2.0 * rho[ip] * t146 + 2.0 * t116;
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
        let t153 = piecewise3(t3, 0.0, -21.0 / 640.0 * t53 * t110 * t22 * t33);
        let tv3sigmatau20 = 2.0 * rho[ip] * t153;
        v3sigmatau2[ip] += tv3sigmatau20;
        let tv3lapl30 = 0.0;
        v3lapl3[ip] += tv3lapl30;
        let tv3lapl2tau0 = 0.0;
        v3lapl2tau[ip] += tv3lapl2tau0;
        let tv3lapltau20 = 0.0;
        v3lapltau2[ip] += tv3lapltau20;
        let t160 = 1.0 / t109 / tau[ip];
        let t166 = piecewise3(t3, 0.0, 81.0 / 80.0 * t17 * t18 * t110 * t34 + 63.0 / 640.0 * t17 * rho[ip] * t160 * t46);
        let tv3tau30 = 2.0 * rho[ip] * t166;
        v3tau3[ip] += tv3tau30;
        let tv4rho40 = 8.0 * t119 + t121;
        v4rho4[ip] += tv4rho40;
        let tv4rho3sigma0 = 6.0 * t119 + t121;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let tv4rho3lapl0 = 0.0;
        v4rho3lapl[ip] += tv4rho3lapl0;
        let tv4rho3tau0 = 6.0 * t131 + t121;
        v4rho3tau[ip] += tv4rho3tau0;
        let tv4rho2sigma20 = 0.0;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let tv4rho2sigmalapl0 = 0.0;
        v4rho2sigmalapl[ip] += tv4rho2sigmalapl0;
        let tv4rho2sigmatau0 = 4.0 * t137 + t121;
        v4rho2sigmatau[ip] += tv4rho2sigmatau0;
        let tv4rho2lapl20 = 0.0;
        v4rho2lapl2[ip] += tv4rho2lapl20;
        let tv4rho2lapltau0 = 0.0;
        v4rho2lapltau[ip] += tv4rho2lapltau0;
        let t180 = piecewise3(t3, 0.0, -27.0 / 40.0 * t134 * t34 + 7.0 / 320.0 * t17 * t23 * t110 * t46);
        let tv4rho2tau20 = 2.0 * rho[ip] * t180 + 4.0 * t146;
        v4rho2tau2[ip] += tv4rho2tau20;
        let tv4rhosigma30 = 0.0;
        v4rhosigma3[ip] += tv4rhosigma30;
        let tv4rhosigma2lapl0 = 0.0;
        v4rhosigma2lapl[ip] += tv4rhosigma2lapl0;
        let tv4rhosigma2tau0 = 0.0;
        v4rhosigma2tau[ip] += tv4rhosigma2tau0;
        let tv4rhosigmalapl20 = 0.0;
        v4rhosigmalapl2[ip] += tv4rhosigmalapl20;
        let tv4rhosigmalapltau0 = 0.0;
        v4rhosigmalapltau[ip] += tv4rhosigmalapltau0;
        let t185 = piecewise3(t3, 0.0, -21.0 / 640.0 * t142 * t86);
        let tv4rhosigmatau20 = 2.0 * rho[ip] * t185 + 2.0 * t153;
        v4rhosigmatau2[ip] += tv4rhosigmatau20;
        let tv4rholapl30 = 0.0;
        v4rholapl3[ip] += tv4rholapl30;
        let tv4rholapl2tau0 = 0.0;
        v4rholapl2tau[ip] += tv4rholapl2tau0;
        let tv4rholapltau20 = 0.0;
        v4rholapltau2[ip] += tv4rholapltau20;
        let t194 = piecewise3(t3, 0.0, 81.0 / 40.0 * t112 * t34 + 21.0 / 320.0 * t17 * t160 * t46);
        let tv4rhotau30 = 2.0 * rho[ip] * t194 + 2.0 * t166;
        v4rhotau3[ip] += tv4rhotau30;
        let tv4sigma40 = 0.0;
        v4sigma4[ip] += tv4sigma40;
        let tv4sigma3lapl0 = 0.0;
        v4sigma3lapl[ip] += tv4sigma3lapl0;
        let tv4sigma3tau0 = 0.0;
        v4sigma3tau[ip] += tv4sigma3tau0;
        let tv4sigma2lapl20 = 0.0;
        v4sigma2lapl2[ip] += tv4sigma2lapl20;
        let tv4sigma2lapltau0 = 0.0;
        v4sigma2lapltau[ip] += tv4sigma2lapltau0;
        let tv4sigma2tau20 = 0.0;
        v4sigma2tau2[ip] += tv4sigma2tau20;
        let tv4sigmalapl30 = 0.0;
        v4sigmalapl3[ip] += tv4sigmalapl30;
        let tv4sigmalapl2tau0 = 0.0;
        v4sigmalapl2tau[ip] += tv4sigmalapl2tau0;
        let tv4sigmalapltau20 = 0.0;
        v4sigmalapltau2[ip] += tv4sigmalapltau20;
        let t201 = piecewise3(t3, 0.0, 21.0 / 160.0 * t53 * t160 * t22 * t33);
        let tv4sigmatau30 = 2.0 * rho[ip] * t201;
        v4sigmatau3[ip] += tv4sigmatau30;
        let tv4lapl40 = 0.0;
        v4lapl4[ip] += tv4lapl40;
        let tv4lapl3tau0 = 0.0;
        v4lapl3tau[ip] += tv4lapl3tau0;
        let tv4lapl2tau20 = 0.0;
        v4lapl2tau2[ip] += tv4lapl2tau20;
        let tv4lapltau30 = 0.0;
        v4lapltau3[ip] += tv4lapltau30;
        let t214 = piecewise3(t3, 0.0, -81.0 / 20.0 * t17 * t18 * t160 * t34 - 21.0 / 40.0 * t17 * rho[ip] / t109 / t42 * t46);
        let tv4tau40 = 2.0 * rho[ip] * t214;
        v4tau4[ip] += tv4tau40;
    }
}
