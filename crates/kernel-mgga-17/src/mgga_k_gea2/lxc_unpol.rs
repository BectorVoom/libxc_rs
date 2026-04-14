//! MGGA_K_GEA2 lxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 32 shared lines across all orders.
//! Delta: 38 lines unique to lxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_k_gea2_lxc_unpol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (32 lines) ---
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = t4 * t4;
        let t6 = M_CBRTPI;
        let t8 = t5 * t6 * M_PI;
        let t9 = 1.0 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t12 = piecewise5(t9, t10, t9, -t10, 0.0);
        let t13 = 1.0 + t12;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * t15;
        let t18 = pow_1_3(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3(t13 <= zeta_threshold, t16 * zeta_threshold, t19 * t13);
        let t22 = pow_1_3(rho[ip]);
        let t23 = t22 * t22;
        let t24 = t21 * t23;
        let t25 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t30 = t25 * t29;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t23 / t34;
        let t40 = lapl[ip] * t32;
        let t46 = 1.0 + 5.0 / 648.0 * t30 * t33 * t36 + 5.0 / 54.0 * t30 * t40 / t23 / rho[ip];
        let t50 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t46);
        let tzk0 = 2.0 * t50;
        zk[ip] += tzk0;
        // --- vxc delta (14 lines) ---
        let t52 = t21 / t22;
        let t56 = t34 * rho[ip];
        let t58 = 1.0 / t23 / t56;
        let t65 = -5.0 / 243.0 * t30 * t33 * t58 - 25.0 / 162.0 * t30 * t40 * t36;
        let t70 = piecewise3(t3, 0.0, t8 * t52 * t46 / 10.0 + 3.0 / 20.0 * t8 * t24 * t65);
        let tvrho0 = 2.0 * rho[ip] * t70 + 2.0 * t50;
        vrho[ip] += tvrho0;
        let t73 = t8 * t21;
        let t76 = t29 * t32;
        let t78 = t73 / t34 * t25 * t76;
        let t80 = piecewise3(t3, 0.0, t78 / 864.0);
        let tvsigma0 = 2.0 * rho[ip] * t80;
        vsigma[ip] += tvsigma0;
        let t87 = piecewise3(t3, 0.0, t73 / rho[ip] * t25 * t76 / 72.0);
        let tvlapl0 = 2.0 * rho[ip] * t87;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
        // --- fxc delta (18 lines) ---
        let t92 = t21 / t22 / rho[ip];
        let t99 = t34 * t34;
        let t101 = 1.0 / t23 / t99;
        let t108 = 55.0 / 729.0 * t30 * t33 * t101 + 100.0 / 243.0 * t30 * t40 * t58;
        let t113 = piecewise3(t3, 0.0, -t8 * t92 * t46 / 30.0 + t8 * t52 * t65 / 5.0 + 3.0 / 20.0 * t8 * t24 * t108);
        let tv2rho20 = 2.0 * rho[ip] * t113 + 4.0 * t70;
        v2rho2[ip] += tv2rho20;
        let t119 = t73 / t56 * t25 * t76;
        let t121 = piecewise3(t3, 0.0, -t119 / 432.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t121 + 2.0 * t80;
        v2rhosigma[ip] += tv2rhosigma0;
        let t125 = piecewise3(t3, 0.0, -t78 / 72.0);
        let tv2rholapl0 = 2.0 * rho[ip] * t125 + 2.0 * t87;
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
        // --- kxc delta (28 lines) ---
        let t131 = t21 / t22 / t34;
        let t141 = t99 * rho[ip];
        let t143 = 1.0 / t23 / t141;
        let t150 = -770.0 / 2187.0 * t30 * t33 * t143 - 1100.0 / 729.0 * t30 * t40 * t101;
        let t155 = piecewise3(t3, 0.0, 2.0 / 45.0 * t8 * t131 * t46 - t8 * t92 * t65 / 10.0 + 3.0 / 10.0 * t8 * t52 * t108 + 3.0 / 20.0 * t8 * t24 * t150);
        let tv3rho30 = 2.0 * rho[ip] * t155 + 6.0 * t113;
        v3rho3[ip] += tv3rho30;
        let t162 = t73 / t99 * t25 * t76;
        let t164 = piecewise3(t3, 0.0, t162 / 144.0);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t164 + 4.0 * t121;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t169 = piecewise3(t3, 0.0, t119 / 36.0);
        let tv3rho2lapl0 = 2.0 * rho[ip] * t169 + 4.0 * t125;
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
        // --- lxc delta (this level) (38 lines) ---
        let t202 = piecewise3(t3, 0.0, -14.0 / 135.0 * t8 * t21 / t22 / t56 * t46 + 8.0 / 45.0 * t8 * t131 * t65 - t8 * t92 * t108 / 5.0 + 2.0 / 5.0 * t8 * t52 * t150 + 3.0 / 20.0 * t8 * t24 * (13090.0 / 6561.0 * t30 * t33 / t23 / t99 / t34 + 15400.0 / 2187.0 * t30 * t40 * t143));
        let tv4rho40 = 2.0 * rho[ip] * t202 + 8.0 * t155;
        v4rho4[ip] += tv4rho40;
        let t211 = piecewise3(t3, 0.0, -t73 / t141 * t25 * t76 / 36.0);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t211 + 6.0 * t164;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t216 = piecewise3(t3, 0.0, -t162 / 12.0);
        let tv4rho3lapl0 = 2.0 * rho[ip] * t216 + 6.0 * t169;
        v4rho3lapl[ip] += tv4rho3lapl0;
        let tv4rho3tau0 = 0.0;
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
