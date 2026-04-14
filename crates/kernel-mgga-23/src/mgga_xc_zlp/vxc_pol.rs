//! MGGA_XC_ZLP vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 41 shared lines across all orders.
//! Delta: 46 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_xc_zlp_vxc_pol(
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
        // --- shared preamble (41 lines) ---
        let t2 = M_CBRT3;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = t2 * t4;
        let t6 = M_CBRT4;
        let t7 = t6 * t6;
        let t11 = sigma0 + 2.0 * sigma1 + sigma2;
        let t12 = rho0 + rho1;
        let t13 = t12 * t12;
        let t14 = pow_1_3(t12);
        let t15 = t14 * t14;
        let t17 = 1.0 / t15 / t13;
        let t19 = pow_1_3(rho0);
        let t20 = t19 * t19;
        let t22 = 1.0 / t20 / rho0;
        let t23 = lapl0 * t22;
        let t24 = rho0 - rho1;
        let t25 = 1.0 / t12;
        let t26 = t24 * t25;
        let t28 = 1.0 / 2.0 + t26 / 2.0;
        let t29 = pow_1_3(t28);
        let t30 = t29 * t29;
        let t31 = t30 * t28;
        let t33 = pow_1_3(rho1);
        let t34 = t33 * t33;
        let t36 = 1.0 / t34 / rho1;
        let t37 = lapl1 * t36;
        let t39 = 1.0 / 2.0 - t26 / 2.0;
        let t40 = pow_1_3(t39);
        let t41 = t40 * t40;
        let t42 = t41 * t39;
        let t49 = 0.207108e0 * t5 * t7 + 0.5387725e-2 * t5 * t7 * (t11 * t17 / 8.0 - t23 * t31 / 8.0 - t37 * t42 / 8.0);
        let t52 = 1.0 + 0.48849425066691677572e3 / t14;
        let t53 = f64::ln(t52);
        let t56 = 1.0 - 0.2047107e-2 * t53 * t14;
        let t58 = t2 * t2;
        let t59 = t49 * t56 * t58;
        let t60 = 1.0 / t4;
        let t61 = t60 * t6;
        let t62 = t61 * t14;
        let t63 = t59 * t62;
        let tzk0 = -t63 / 3.0;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (46 lines) ---
        let t65 = 4.0 / 9.0 * t63;
        let t66 = t14 * t12;
        let t67 = t13 * t12;
        let t69 = 1.0 / t15 / t67;
        let t71 = t11 * t69 / 3.0;
        let t72 = rho0 * rho0;
        let t74 = 1.0 / t20 / t72;
        let t75 = lapl0 * t74;
        let t78 = 1.0 / t13;
        let t79 = t24 * t78;
        let t81 = t25 / 2.0 - t79 / 2.0;
        let t82 = t30 * t81;
        let t85 = -t81;
        let t86 = t41 * t85;
        let t89 = -t71 + 5.0 / 24.0 * t75 * t31 - 5.0 / 24.0 * t23 * t82 - 5.0 / 24.0 * t37 * t86;
        let t90 = t66 * t89;
        let t93 = t66 * t49;
        let t94 = 1.0 / t52;
        let t97 = 1.0 / t15;
        let t100 = 0.33333333333333333332e0 * t25 * t94 - 0.682369e-3 * t53 * t97;
        let t103 = t58 * t60 * t6;
        let t105 = t93 * t100 * t103 / 3.0;
        let tvrho0 = -t65 - 0.215509e-1 * t90 * t56 - t105;
        vrho[ip * 2] += tvrho0;
        let t107 = -t25 / 2.0 - t79 / 2.0;
        let t108 = t30 * t107;
        let t111 = rho1 * rho1;
        let t113 = 1.0 / t34 / t111;
        let t114 = lapl1 * t113;
        let t117 = -t107;
        let t118 = t41 * t117;
        let t121 = -t71 - 5.0 / 24.0 * t23 * t108 + 5.0 / 24.0 * t114 * t42 - 5.0 / 24.0 * t37 * t118;
        let t122 = t66 * t121;
        let tvrho1 = -t65 - 0.215509e-1 * t122 * t56 - t105;
        vrho[ip * 2 + 1] += tvrho1;
        let t125 = 1.0 / t66;
        let t126 = t125 * t56;
        let tvsigma0 = -0.26938625e-2 * t126;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = -0.5387725e-2 * t126;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = tvsigma0;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t129 = t66 * t22;
        let t130 = t31 * t56;
        let tvlapl0 = 0.26938625e-2 * t129 * t130;
        vlapl[ip * 2] += tvlapl0;
        let t132 = t66 * t36;
        let t133 = t42 * t56;
        let tvlapl1 = 0.26938625e-2 * t132 * t133;
        vlapl[ip * 2 + 1] += tvlapl1;
        let tvtau0 = 0.0;
        vtau[ip * 2] += tvtau0;
        let tvtau1 = 0.0;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
