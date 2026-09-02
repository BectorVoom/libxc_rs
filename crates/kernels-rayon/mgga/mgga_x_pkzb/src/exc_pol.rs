//! MGGA_X_PKZB exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_pkzb.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_pkzb_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = pow_1_3(t7);
        let t29 = M_CBRT6;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t41 = t34 * sigma0 * t39;
        let t44 = 1.0 / t37 / rho0;
        let t49 = t34 * tau0 * t44 / 4.0 - 9.0 / 20.0 - t41 / 288.0;
        let t50 = t49 * t49;
        let t52 = t49 * t29;
        let t53 = t33 * sigma0;
        let t54 = t53 * t39;
        let t57 = t29 * t29;
        let t59 = 1.0 / t31 / t30;
        let t60 = t57 * t59;
        let t61 = sigma0 * sigma0;
        let t62 = t35 * t35;
        let t63 = t62 * rho0;
        let t65 = 1.0 / t36 / t63;
        let t69 = 0.804 + 5.0 / 972.0 * t41 + 146.0 / 2025.0 * t50 - 73.0 / 9720.0 * t52 * t54 + 0.0002290923400091281 * t60 * t61 * t65;
        let t72 = 1.804 - 0.646416 / t69;
        let t76 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t26 * t27 * t72);
        let t77 = rho1 <= dens_threshold;
        let t78 = -t17;
        let t80 = piecewise5(t15, t12, t11, t16, t78 * t8);
        let t81 = 1.0 + t80;
        let t82 = t81 <= zeta_threshold;
        let t83 = pow_1_3(t81);
        let t85 = piecewise3(t82, t23, t83 * t81);
        let t87 = rho1 * rho1;
        let t88 = pow_1_3(rho1);
        let t89 = t88 * t88;
        let t91 = 1.0 / t89 / t87;
        let t93 = t34 * sigma2 * t91;
        let t96 = 1.0 / t89 / rho1;
        let t101 = t34 * tau1 * t96 / 4.0 - 9.0 / 20.0 - t93 / 288.0;
        let t102 = t101 * t101;
        let t104 = t101 * t29;
        let t105 = t33 * sigma2;
        let t106 = t105 * t91;
        let t109 = sigma2 * sigma2;
        let t110 = t87 * t87;
        let t111 = t110 * rho1;
        let t113 = 1.0 / t88 / t111;
        let t117 = 0.804 + 5.0 / 972.0 * t93 + 146.0 / 2025.0 * t102 - 73.0 / 9720.0 * t104 * t106 + 0.0002290923400091281 * t60 * t109 * t113;
        let t120 = 1.804 - 0.646416 / t117;
        let t124 = piecewise3(t77, 0.0, -3.0 / 8.0 * t6 * t85 * t27 * t120);
        let tzk0 = t76 + t124;
        zk[ip] += tzk0;
    }
}
