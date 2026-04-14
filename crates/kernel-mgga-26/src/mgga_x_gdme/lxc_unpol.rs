//! MGGA_X_GDME lxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 33 shared lines across all orders.
//! Delta: 40 lines unique to lxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_gdme_lxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rho2lapl: &mut Array<f64>,
    v3rho2tau: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3rhosigmalapl: &mut Array<f64>,
    v3rhosigmatau: &mut Array<f64>,
    v3rholapl2: &mut Array<f64>,
    v3rholapltau: &mut Array<f64>,
    v3rhotau2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v3sigma2lapl: &mut Array<f64>,
    v3sigma2tau: &mut Array<f64>,
    v3sigmalapl2: &mut Array<f64>,
    v3sigmalapltau: &mut Array<f64>,
    v3sigmatau2: &mut Array<f64>,
    v3lapl3: &mut Array<f64>,
    v3lapl2tau: &mut Array<f64>,
    v3lapltau2: &mut Array<f64>,
    v3tau3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho3lapl: &mut Array<f64>,
    v4rho3tau: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rho2sigmalapl: &mut Array<f64>,
    v4rho2sigmatau: &mut Array<f64>,
    v4rho2lapl2: &mut Array<f64>,
    v4rho2lapltau: &mut Array<f64>,
    v4rho2tau2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4rhosigma2lapl: &mut Array<f64>,
    v4rhosigma2tau: &mut Array<f64>,
    v4rhosigmalapl2: &mut Array<f64>,
    v4rhosigmalapltau: &mut Array<f64>,
    v4rhosigmatau2: &mut Array<f64>,
    v4rholapl3: &mut Array<f64>,
    v4rholapl2tau: &mut Array<f64>,
    v4rholapltau2: &mut Array<f64>,
    v4rhotau3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    v4sigma3lapl: &mut Array<f64>,
    v4sigma3tau: &mut Array<f64>,
    v4sigma2lapl2: &mut Array<f64>,
    v4sigma2lapltau: &mut Array<f64>,
    v4sigma2tau2: &mut Array<f64>,
    v4sigmalapl3: &mut Array<f64>,
    v4sigmalapl2tau: &mut Array<f64>,
    v4sigmalapltau2: &mut Array<f64>,
    v4sigmatau3: &mut Array<f64>,
    v4lapl4: &mut Array<f64>,
    v4lapl3tau: &mut Array<f64>,
    v4lapl2tau2: &mut Array<f64>,
    v4lapltau3: &mut Array<f64>,
    v4tau4: &mut Array<f64>,
    param_AA: f64,
    param_BB: f64,
    param_a: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (33 lines) ---
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
        // --- vxc delta (18 lines) ---
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
        // --- fxc delta (19 lines) ---
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
        // --- kxc delta (29 lines) ---
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
        // --- lxc delta (this level) (40 lines) ---
        let t215 = 1.0 / t46 / t165 / rho[ip];
        let t227 = piecewise3(t3, 0.0, 10.0 / 27.0 * t7 * t18 * t123 * t59 - 5.0 / 243.0 * t70 * t155 * param_BB * t84 + t160 * t131 / 54.0 - t118 * t175 / 54.0 - t72 * t29 * t73 * (6160.0 / 81.0 * t45 * t39 * t215 - 12320.0 / 81.0 * t51 * t215) / 72.0);
        let tv4rho40 = 2.0 * rho[ip] * t227 + 8.0 * t179;
        v4rho4[ip] += tv4rho40;
        let tv4rho3sigma0 = 0.0;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t233 = t18 / t19 / t165;
        let t238 = piecewise3(t3, 0.0, 70.0 / 243.0 * t70 * t233 * param_BB * t98);
        let tv4rho3lapl0 = 2.0 * rho[ip] * t238 + 6.0 * t190;
        v4rho3lapl[ip] += tv4rho3lapl0;
        let t245 = piecewise3(t3, 0.0, -140.0 / 243.0 * t70 * t233 * t107);
        let tv4rho3tau0 = 2.0 * rho[ip] * t245 + 6.0 * t197;
        v4rho3tau[ip] += tv4rho3tau0;
        let tv4rho2sigma20 = 0.0;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let tv4rho2sigmalapl0 = 0.0;
        v4rho2sigmalapl[ip] += tv4rho2sigmalapl0;
        let tv4rho2sigmatau0 = 0.0;
        v4rho2sigmatau[ip] += tv4rho2sigmatau0;
        let tv4rho2lapl20 = 0.0;
        v4rho2lapl2[ip] += tv4rho2lapl20;
        let tv4rho2lapltau0 = 0.0;
        v4rho2lapltau[ip] += tv4rho2lapltau0;
        let tv4rho2tau20 = 0.0;
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
        let tv4rhosigmatau20 = 0.0;
        v4rhosigmatau2[ip] += tv4rhosigmatau20;
        let tv4rholapl30 = 0.0;
        v4rholapl3[ip] += tv4rholapl30;
        let tv4rholapl2tau0 = 0.0;
        v4rholapl2tau[ip] += tv4rholapl2tau0;
        let tv4rholapltau20 = 0.0;
        v4rholapltau2[ip] += tv4rholapltau20;
        let tv4rhotau30 = 0.0;
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
        let tv4sigmatau30 = 0.0;
        v4sigmatau3[ip] += tv4sigmatau30;
        let tv4lapl40 = 0.0;
        v4lapl4[ip] += tv4lapl40;
        let tv4lapl3tau0 = 0.0;
        v4lapl3tau[ip] += tv4lapl3tau0;
        let tv4lapl2tau20 = 0.0;
        v4lapl2tau2[ip] += tv4lapl2tau20;
        let tv4lapltau30 = 0.0;
        v4lapltau3[ip] += tv4lapltau30;
        let tv4tau40 = 0.0;
        v4tau4[ip] += tv4tau40;
    }
}
