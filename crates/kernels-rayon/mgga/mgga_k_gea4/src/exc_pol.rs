//! MGGA_K_GEA4 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_gea4.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_k_gea4_exc_pol(
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
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = rho0 + rho1;
        let t9 = 1.0 / t8;
        let t12 = 2.0 * rho0 * t9 <= zeta_threshold;
        let t13 = zeta_threshold - 1.0;
        let t16 = 2.0 * rho1 * t9 <= zeta_threshold;
        let t17 = -t13;
        let t18 = rho0 - rho1;
        let t20 = piecewise5(t12, t13, t16, t17, t18 * t9);
        let t21 = 1.0 + t20;
        let t22 = t21 <= zeta_threshold;
        let t23 = pow_1_3(zeta_threshold);
        let t24 = t23 * t23;
        let t25 = t24 * zeta_threshold;
        let t26 = pow_1_3(t21);
        let t27 = t26 * t26;
        let t29 = piecewise3(t22, t25, t27 * t21);
        let t30 = pow_1_3(t8);
        let t31 = t30 * t30;
        let t32 = t29 * t31;
        let t33 = M_CBRT6;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t38 = t33 / t36;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t48 = 1.0 / t41 / rho0;
        let t52 = t33 * t33;
        let t54 = 1.0 / t35 / t34;
        let t55 = t52 * t54;
        let t56 = lapl0 * lapl0;
        let t57 = t39 * rho0;
        let t59 = 1.0 / t40 / t57;
        let t63 = t39 * t39;
        let t65 = 1.0 / t40 / t63;
        let t66 = sigma0 * t65;
        let t70 = sigma0 * sigma0;
        let t71 = t63 * rho0;
        let t73 = 1.0 / t40 / t71;
        let t77 = 1.0 + 5.0 / 648.0 * t38 * sigma0 * t43 + 5.0 / 54.0 * t38 * lapl0 * t48 + t55 * t56 * t59 / 5832.0 - t55 * t66 * lapl0 / 5184.0 + t55 * t70 * t73 / 17496.0;
        let t81 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t32 * t77);
        let t82 = rho1 <= dens_threshold;
        let t83 = -t18;
        let t85 = piecewise5(t16, t13, t12, t17, t83 * t9);
        let t86 = 1.0 + t85;
        let t87 = t86 <= zeta_threshold;
        let t88 = pow_1_3(t86);
        let t89 = t88 * t88;
        let t91 = piecewise3(t87, t25, t89 * t86);
        let t92 = t91 * t31;
        let t93 = rho1 * rho1;
        let t94 = pow_1_3(rho1);
        let t95 = t94 * t94;
        let t97 = 1.0 / t95 / t93;
        let t102 = 1.0 / t95 / rho1;
        let t106 = lapl1 * lapl1;
        let t107 = t93 * rho1;
        let t109 = 1.0 / t94 / t107;
        let t113 = t93 * t93;
        let t115 = 1.0 / t94 / t113;
        let t116 = sigma2 * t115;
        let t120 = sigma2 * sigma2;
        let t121 = t113 * rho1;
        let t123 = 1.0 / t94 / t121;
        let t127 = 1.0 + 5.0 / 648.0 * t38 * sigma2 * t97 + 5.0 / 54.0 * t38 * lapl1 * t102 + t55 * t106 * t109 / 5832.0 - t55 * t116 * lapl1 / 5184.0 + t55 * t120 * t123 / 17496.0;
        let t131 = piecewise3(t82, 0.0, 3.0 / 20.0 * t7 * t92 * t127);
        let tzk0 = t81 + t131;
        zk[ip] += tzk0;
    }
}
