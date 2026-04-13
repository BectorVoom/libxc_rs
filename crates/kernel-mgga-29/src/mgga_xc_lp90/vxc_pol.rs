//! MGGA_XC_LP90 vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_lp90.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_xc_lp90_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t3 = sigma0 + 2.0 * sigma1 + sigma2;
        let t4 = rho0 + rho1;
        let t5 = t4 * t4;
        let t6 = pow_1_3(t4);
        let t7 = t6 * t6;
        let t9 = 1.0 / t7 / t5;
        let t12 = pow_1_3(rho0);
        let t13 = t12 * t12;
        let t15 = 1.0 / t13 / rho0;
        let t16 = lapl0 * t15;
        let t17 = rho0 - rho1;
        let t18 = 1.0 / t4;
        let t19 = t17 * t18;
        let t21 = 1.0 / 2.0 + t19 / 2.0;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = t23 * t21;
        let t27 = pow_1_3(rho1);
        let t28 = t27 * t27;
        let t30 = 1.0 / t28 / rho1;
        let t31 = lapl1 * t30;
        let t33 = 1.0 / 2.0 - t19 / 2.0;
        let t34 = pow_1_3(t33);
        let t35 = t34 * t34;
        let t36 = t35 * t33;
        let t39 = 0.80569e0 + 0.37655e-3 * t3 * t9 - 0.37655e-3 * t16 * t24 - 0.37655e-3 * t31 * t36;
        let t40 = 1.0 / t6;
        let t41 = t40 + 0.40743e-2;
        let t42 = 1.0 / t41;
        let tzk0 = -t39 * t42;
        zk[ip] += tzk0;
        let t44 = t5 * t4;
        let t46 = 1.0 / t7 / t44;
        let t48 = 0.10041333333333333333e-2 * t3 * t46;
        let t49 = rho0 * rho0;
        let t51 = 1.0 / t13 / t49;
        let t52 = lapl0 * t51;
        let t55 = 1.0 / t5;
        let t56 = t17 * t55;
        let t58 = t18 / 2.0 - t56 / 2.0;
        let t59 = t23 * t58;
        let t62 = -t58;
        let t63 = t35 * t62;
        let t66 = -t48 + 0.62758333333333333333e-3 * t52 * t24 - 0.62758333333333333333e-3 * t16 * t59 - 0.62758333333333333333e-3 * t31 * t63;
        let t70 = t41 * t41;
        let t71 = 1.0 / t70;
        let t73 = t40 * t39 * t71 / 3.0;
        let tvrho0 = -t4 * t66 * t42 - t73 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t75 = -t18 / 2.0 - t56 / 2.0;
        let t76 = t23 * t75;
        let t79 = rho1 * rho1;
        let t81 = 1.0 / t28 / t79;
        let t82 = lapl1 * t81;
        let t85 = -t75;
        let t86 = t35 * t85;
        let t89 = -t48 - 0.62758333333333333333e-3 * t16 * t76 + 0.62758333333333333333e-3 * t82 * t36 - 0.62758333333333333333e-3 * t31 * t86;
        let tvrho1 = -t4 * t89 * t42 - t73 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
        let t93 = 1.0 / t7 / t4;
        let t94 = t93 * t42;
        let tvsigma0 = -0.37655e-3 * t94;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = -0.7531e-3 * t94;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = tvsigma0;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t97 = t4 * t15;
        let t98 = t24 * t42;
        let tvlapl0 = 0.37655e-3 * t97 * t98;
        vlapl[ip * 2] += tvlapl0;
        let t100 = t4 * t30;
        let t101 = t36 * t42;
        let tvlapl1 = 0.37655e-3 * t100 * t101;
        vlapl[ip * 2 + 1] += tvlapl1;
        let tvtau0 = 0.0;
        vtau[ip * 2] += tvtau0;
        let tvtau1 = 0.0;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
