//! MGGA_XC_ZLP lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_zlp.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_xc_zlp_lxc_unpol(
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
        let t2 = M_CBRT3;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = t2 * t4;
        let t6 = M_CBRT4;
        let t7 = t6 * t6;
        let t10 = rho[ip] * rho[ip];
        let t11 = pow_1_3(rho[ip]);
        let t12 = t11 * t11;
        let t14 = 1.0 / t12 / t10;
        let t17 = 1.0 / t12 / rho[ip];
        let t24 = 0.207108 * t5 * t7 + 0.005387725 * t5 * t7 * (-lapl[ip] * t17 / 8.0 + sigma[ip] * t14 / 8.0);
        let t25 = 1.0 / t11;
        let t27 = 1.0 + 488.4942506669168 * t25;
        let t28 = rmath::ln(t27);
        let t31 = 1.0 - 0.002047107 * t28 * t11;
        let t33 = t2 * t2;
        let t34 = t24 * t31 * t33;
        let t35 = 1.0 / t4;
        let t36 = t35 * t6;
        let t37 = t36 * t11;
        let t38 = t34 * t37;
        let tzk0 = -t38 / 3.0;
        zk[ip] += tzk0;
        let t41 = t11 * rho[ip];
        let t42 = t10 * rho[ip];
        let t44 = 1.0 / t12 / t42;
        let t49 = -sigma[ip] * t44 / 3.0 + 5.0 / 24.0 * lapl[ip] * t14;
        let t50 = t41 * t49;
        let t53 = t41 * t24;
        let t55 = 1.0 / t27;
        let t58 = 1.0 / t12;
        let t61 = 0.3333333333333333 / rho[ip] * t55 - 0.000682369 * t28 * t58;
        let t64 = t33 * t35 * t6;
        let tvrho0 = -4.0 / 9.0 * t38 - 0.0215509 * t50 * t31 - t53 * t61 * t64 / 3.0;
        vrho[ip] += tvrho0;
        let t67 = 1.0 / t41;
        let t68 = t67 * t31;
        let tvsigma0 = -0.0026938625 * t68;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0026938625 * t25 * t31;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
        let t71 = t49 * t31;
        let t75 = t24 * t61 * t33;
        let t78 = t36 * t58;
        let t81 = t10 * t10;
        let t83 = 1.0 / t12 / t81;
        let t88 = 11.0 / 9.0 * sigma[ip] * t83 - 5.0 / 9.0 * lapl[ip] * t44;
        let t89 = t41 * t88;
        let t98 = 1.0 / t11 / t10;
        let t99 = t27 * t27;
        let t100 = 1.0 / t99;
        let t105 = -0.2222222222222222 / t10 * t55 + 54.277138962990755 * t98 * t100 + 0.00045491266666666667 * t28 * t17;
        let tv2rho20 = -0.057469066666666666 * t71 * t11 - 8.0 / 9.0 * t75 * t37 - 4.0 / 27.0 * t34 * t78 - 0.0215509 * t89 * t31 - 0.0431018 * t50 * t61 - t53 * t105 * t64 / 3.0;
        v2rho2[ip] += tv2rho20;
        let t109 = t98 * t31;
        let t111 = t67 * t61;
        let tv2rhosigma0 = 0.0035918166666666666 * t109 - 0.0026938625 * t111;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = -0.0008979541666666666 * t68 + 0.0026938625 * t25 * t61;
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
        let t116 = t88 * t31;
        let t119 = t49 * t61;
        let t125 = t24 * t105 * t33;
        let t130 = t36 * t17;
        let t133 = t81 * rho[ip];
        let t135 = 1.0 / t12 / t133;
        let t140 = -154.0 / 27.0 * sigma[ip] * t135 + 55.0 / 27.0 * lapl[ip] * t83;
        let t141 = t41 * t140;
        let t152 = 1.0 / t11 / t42;
        let t156 = 1.0 / t99 / t27;
        let t161 = 0.37037037037037035 / t42 * t55 - 162.83141688897226 * t152 * t100 + 17676.046884046853 * t44 * t156 - 0.0007581877777777777 * t28 * t14;
        let tv3rho30 = -0.0862036 * t116 * t11 - 0.1724072 * t119 * t11 - 0.028734533333333333 * t71 * t58 - 4.0 / 3.0 * t125 * t37 - 4.0 / 9.0 * t75 * t78 + 8.0 / 81.0 * t34 * t130 - 0.0215509 * t141 * t31 - 0.0646527 * t89 * t61 - 0.0646527 * t50 * t105 - t53 * t161 * t64 / 3.0;
        v3rho3[ip] += tv3rho30;
        let t165 = t152 * t31;
        let t167 = t98 * t61;
        let t170 = 0.0026938625 * t67 * t105;
        let tv3rho2sigma0 = -0.008380905555555556 * t165 + 0.007183633333333333 * t167 - t170;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let tv3rho2lapl0 = 0.0011972722222222223 * t109 - 0.0017959083333333333 * t111 + 0.0026938625 * t25 * t105;
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
        let t218 = 1.0 / t11 / t81;
        let t224 = t99 * t99;
        let tv4rho40 = -0.3448144 * t49 * t105 * t11 - 0.0215509 * t41 * (2618.0 / 81.0 * sigma[ip] / t12 / t81 / t10 - 770.0 / 81.0 * lapl[ip] * t135) * t31 - 0.0862036 * t141 * t61 - 0.1293054 * t89 * t105 - 0.0862036 * t50 * t161 - 0.11493813333333333 * t140 * t31 * t11 - 0.3448144 * t88 * t61 * t11 - 0.057469066666666666 * t116 * t58 - 0.11493813333333333 * t119 * t58 + 0.025541807407407406 * t71 * t17 - 8.0 / 9.0 * t125 * t78 + 32.0 / 81.0 * t75 * t130 - 40.0 / 243.0 * t34 * t36 * t14 - t53 * (-0.9876543209876543 / t81 * t55 + 603.0793218110084 * t218 * t100 - 117840.31256031236 * t83 * t156 + 8634647.277375758 / t133 / t224 + 0.0020218340740740743 * t28 * t44) * t64 / 3.0 - 16.0 / 9.0 * t24 * t161 * t33 * t37;
        v4rho4[ip] += tv4rho40;
        let tv4rho3sigma0 = 0.02793635185185185 * t218 * t31 - 0.025142716666666665 * t152 * t61 + 0.01077545 * t98 * t105 - 0.0026938625 * t67 * t161;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let tv4rho3lapl0 = -0.0027936351851851854 * t165 + 0.0035918166666666666 * t167 - t170 + 0.0026938625 * t25 * t161;
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
