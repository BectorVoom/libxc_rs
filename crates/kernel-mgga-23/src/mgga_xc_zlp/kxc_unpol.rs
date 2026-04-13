//! MGGA_XC_ZLP kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_zlp.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_xc_zlp_kxc_unpol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
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
        let t24 = 0.207108e0 * t5 * t7 + 0.5387725e-2 * t5 * t7 * (-lapl[ip] * t17 / 8.0 + sigma[ip] * t14 / 8.0);
        let t25 = 1.0 / t11;
        let t27 = 1.0 + 0.48849425066691677572e3 * t25;
        let t28 = f64::ln(t27);
        let t31 = 1.0 - 0.2047107e-2 * t28 * t11;
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
        let t61 = 0.33333333333333333332e0 / rho[ip] * t55 - 0.682369e-3 * t28 * t58;
        let t64 = t33 * t35 * t6;
        let tvrho0 = -4.0 / 9.0 * t38 - 0.215509e-1 * t50 * t31 - t53 * t61 * t64 / 3.0;
        vrho[ip] += tvrho0;
        let t67 = 1.0 / t41;
        let t68 = t67 * t31;
        let tvsigma0 = -0.26938625e-2 * t68;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.26938625e-2 * t25 * t31;
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
        let t105 = -0.22222222222222222221e0 / t10 * t55 + 0.54277138962990752854e2 * t98 * t100 + 0.45491266666666666667e-3 * t28 * t17;
        let tv2rho20 = -0.57469066666666666666e-1 * t71 * t11 - 8.0 / 9.0 * t75 * t37 - 4.0 / 27.0 * t34 * t78 - 0.215509e-1 * t89 * t31 - 0.431018e-1 * t50 * t61 - t53 * t105 * t64 / 3.0;
        v2rho2[ip] += tv2rho20;
        let t109 = t98 * t31;
        let t111 = t67 * t61;
        let tv2rhosigma0 = 0.35918166666666666667e-2 * t109 - 0.26938625e-2 * t111;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = -0.89795416666666666667e-3 * t68 + 0.26938625e-2 * t25 * t61;
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
        let t161 = 0.37037037037037037035e0 / t42 * t55 - 0.16283141688897225856e3 * t152 * t100 + 0.1767604688404685339e5 * t44 * t156 - 0.75818777777777777778e-3 * t28 * t14;
        let tv3rho30 = -0.86203599999999999999e-1 * t116 * t11 - 0.1724072e0 * t119 * t11 - 0.28734533333333333333e-1 * t71 * t58 - 4.0 / 3.0 * t125 * t37 - 4.0 / 9.0 * t75 * t78 + 8.0 / 81.0 * t34 * t130 - 0.215509e-1 * t141 * t31 - 0.646527e-1 * t89 * t61 - 0.646527e-1 * t50 * t105 - t53 * t161 * t64 / 3.0;
        v3rho3[ip] += tv3rho30;
        let t165 = t152 * t31;
        let t167 = t98 * t61;
        let t170 = 0.26938625e-2 * t67 * t105;
        let tv3rho2sigma0 = -0.83809055555555555556e-2 * t165 + 0.71836333333333333334e-2 * t167 - t170;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let tv3rho2lapl0 = 0.11972722222222222222e-2 * t109 - 0.17959083333333333333e-2 * t111 + 0.26938625e-2 * t25 * t105;
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
