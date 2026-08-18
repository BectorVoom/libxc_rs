//! MGGA_XC_ZLP fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_zlp.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_xc_zlp_fxc_unpol(
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
        let t28 = f64::ln(t27);
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
    }
}
