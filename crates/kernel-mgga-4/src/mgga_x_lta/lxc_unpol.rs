//! MGGA_X_LTA lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_lta.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_lta_lxc_unpol(
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
    param_ltafrac: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t21 = M_CBRT2;
        let t22 = t21 * t21;
        let t24 = t19 * t19;
        let t26 = 1.0 / t24 / rho[ip];
        let t27 = M_CBRT6;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = t30 * t30;
        let t37 = f64::powf(5.0 / 9.0 * tau[ip] * t22 * t26 * t27 / t31, 4.0 / 5.0 * param_ltafrac);
        let t41 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t18 * t19 * t37);
        let tzk0 = 2.0 * t41;
        zk[ip] += tzk0;
        let t42 = 1.0 / t24;
        let t47 = t7 * t18;
        let t48 = t42 * t37;
        let t53 = piecewise3(t3, 0.0, -t7 * t18 * t42 * t37 / 8.0 + t47 * t48 * param_ltafrac / 2.0);
        let tvrho0 = 2.0 * rho[ip] * t53 + 2.0 * t41;
        vrho[ip] += tvrho0;
        let tvsigma0 = 0.0;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t56 = t19 * t37;
        let t57 = 1.0 / tau[ip];
        let t58 = param_ltafrac * t57;
        let t62 = piecewise3(t3, 0.0, -3.0 / 10.0 * t47 * t56 * t58);
        let tvtau0 = 2.0 * rho[ip] * t62;
        vtau[ip] += tvtau0;
        let t69 = t26 * t37;
        let t73 = param_ltafrac * param_ltafrac;
        let t78 = piecewise3(t3, 0.0, t7 * t18 * t26 * t37 / 12.0 - t47 * t69 * param_ltafrac / 6.0 - 2.0 / 3.0 * t47 * t69 * t73);
        let tv2rho20 = 2.0 * rho[ip] * t78 + 4.0 * t53;
        v2rho2[ip] += tv2rho20;
        let tv2rhosigma0 = 0.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t84 = t73 * t57;
        let t89 = piecewise3(t3, 0.0, -t47 * t48 * t58 / 10.0 + 2.0 / 5.0 * t47 * t48 * t84);
        let tv2rhotau0 = 2.0 * rho[ip] * t89 + 2.0 * t62;
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
        let t92 = tau[ip] * tau[ip];
        let t93 = 1.0 / t92;
        let t94 = t73 * t93;
        let t98 = param_ltafrac * t93;
        let t103 = piecewise3(t3, 0.0, -6.0 / 25.0 * t47 * t56 * t94 + 3.0 / 10.0 * t47 * t56 * t98);
        let tv2tau20 = 2.0 * rho[ip] * t103;
        v2tau2[ip] += tv2tau20;
        let t106 = rho[ip] * rho[ip];
        let t108 = 1.0 / t24 / t106;
        let t113 = t108 * t37;
        let t120 = t73 * param_ltafrac;
        let t125 = piecewise3(t3, 0.0, -5.0 / 36.0 * t7 * t18 * t108 * t37 + t47 * t113 * param_ltafrac / 6.0 + 4.0 / 3.0 * t47 * t113 * t73 + 8.0 / 9.0 * t47 * t113 * t120);
        let tv3rho30 = 2.0 * rho[ip] * t125 + 6.0 * t78;
        v3rho3[ip] += tv3rho30;
        let tv3rho2sigma0 = 0.0;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let tv3rho2lapl0 = 0.0;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let t135 = t120 * t57;
        let t140 = piecewise3(t3, 0.0, t47 * t69 * t58 / 15.0 - 2.0 / 15.0 * t47 * t69 * t84 - 8.0 / 15.0 * t47 * t69 * t135);
        let tv3rho2tau0 = 2.0 * rho[ip] * t140 + 4.0 * t89;
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
        let t146 = t120 * t93;
        let t154 = piecewise3(t3, 0.0, -12.0 / 25.0 * t47 * t48 * t94 + 8.0 / 25.0 * t47 * t48 * t146 + t47 * t48 * t98 / 10.0);
        let tv3rhotau20 = 2.0 * rho[ip] * t154 + 2.0 * t103;
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
        let t158 = 1.0 / t92 / tau[ip];
        let t159 = t120 * t158;
        let t163 = t73 * t158;
        let t167 = param_ltafrac * t158;
        let t172 = piecewise3(t3, 0.0, -24.0 / 125.0 * t47 * t56 * t159 + 18.0 / 25.0 * t47 * t56 * t163 - 3.0 / 5.0 * t47 * t56 * t167);
        let tv3tau30 = 2.0 * rho[ip] * t172;
        v3tau3[ip] += tv3tau30;
        let t177 = 1.0 / t24 / t106 / rho[ip];
        let t182 = t177 * t37;
        let t192 = t73 * t73;
        let t197 = piecewise3(t3, 0.0, 10.0 / 27.0 * t7 * t18 * t177 * t37 - 7.0 / 27.0 * t47 * t182 * param_ltafrac - 34.0 / 9.0 * t47 * t182 * t73 - 112.0 / 27.0 * t47 * t182 * t120 - 32.0 / 27.0 * t47 * t182 * t192);
        let tv4rho40 = 2.0 * rho[ip] * t197 + 8.0 * t125;
        v4rho4[ip] += tv4rho40;
        let tv4rho3sigma0 = 0.0;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let tv4rho3lapl0 = 0.0;
        v4rho3lapl[ip] += tv4rho3lapl0;
        let t215 = piecewise3(t3, 0.0, -t47 * t113 * t58 / 9.0 + 2.0 / 15.0 * t47 * t113 * t84 + 16.0 / 15.0 * t47 * t113 * t135 + 32.0 / 45.0 * t47 * t113 * t192 * t57);
        let tv4rho3tau0 = 2.0 * rho[ip] * t215 + 6.0 * t140;
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
        let t233 = piecewise3(t3, 0.0, 14.0 / 75.0 * t47 * t69 * t94 + 32.0 / 75.0 * t47 * t69 * t146 - 32.0 / 75.0 * t47 * t69 * t192 * t93 - t47 * t69 * t98 / 15.0);
        let tv4rho2tau20 = 2.0 * rho[ip] * t233 + 4.0 * t154;
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
        let t250 = piecewise3(t3, 0.0, -128.0 / 125.0 * t47 * t48 * t159 + 32.0 / 125.0 * t47 * t48 * t192 * t158 + 26.0 / 25.0 * t47 * t48 * t163 - t47 * t48 * t167 / 5.0);
        let tv4rhotau30 = 2.0 * rho[ip] * t250 + 2.0 * t172;
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
        let t253 = t92 * t92;
        let t254 = 1.0 / t253;
        let t272 = piecewise3(t3, 0.0, -96.0 / 625.0 * t47 * t56 * t192 * t254 + 144.0 / 125.0 * t47 * t56 * t120 * t254 - 66.0 / 25.0 * t47 * t56 * t73 * t254 + 9.0 / 5.0 * t47 * t56 * param_ltafrac * t254);
        let tv4tau40 = 2.0 * rho[ip] * t272;
        v4tau4[ip] += tv4tau40;
    }
}
