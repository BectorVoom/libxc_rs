//! GGA_K_PEARSON exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_pearson.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_pearson_exc_pol(
    rho: &[f64],
    sigma: &[f64],
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
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = t2 * t2;
        let t4 = M_CBRTPI;
        let t6 = t3 * t4 * M_PI;
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
        let t23 = t22 * t22;
        let t24 = t23 * zeta_threshold;
        let t25 = pow_1_3(t20);
        let t26 = t25 * t25;
        let t28 = piecewise3(t21, t24, t26 * t20);
        let t29 = pow_1_3(t7);
        let t30 = t29 * t29;
        let t31 = t28 * t30;
        let t32 = M_CBRT6;
        let t33 = M_PI * M_PI;
        let t34 = pow_1_3(t33);
        let t35 = t34 * t34;
        let t37 = t32 / t35;
        let t38 = rho0 * rho0;
        let t39 = pow_1_3(rho0);
        let t40 = t39 * t39;
        let t42 = 1.0 / t40 / t38;
        let t44 = t33 * t33;
        let t45 = 1.0 / t44;
        let t46 = sigma0 * sigma0;
        let t47 = t46 * sigma0;
        let t49 = t38 * t38;
        let t50 = t49 * t49;
        let t54 = 1.0 + t45 * t47 / t50 / 2304.0;
        let t55 = 1.0 / t54;
        let t59 = 1.0 + 5.0 / 648.0 * t37 * sigma0 * t42 * t55;
        let t63 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t59);
        let t64 = rho1 <= dens_threshold;
        let t65 = -t17;
        let t67 = piecewise5(t15, t12, t11, t16, t65 * t8);
        let t68 = 1.0 + t67;
        let t69 = t68 <= zeta_threshold;
        let t70 = pow_1_3(t68);
        let t71 = t70 * t70;
        let t73 = piecewise3(t69, t24, t71 * t68);
        let t74 = t73 * t30;
        let t75 = rho1 * rho1;
        let t76 = pow_1_3(rho1);
        let t77 = t76 * t76;
        let t79 = 1.0 / t77 / t75;
        let t81 = sigma2 * sigma2;
        let t82 = t81 * sigma2;
        let t84 = t75 * t75;
        let t85 = t84 * t84;
        let t89 = 1.0 + t45 * t82 / t85 / 2304.0;
        let t90 = 1.0 / t89;
        let t94 = 1.0 + 5.0 / 648.0 * t37 * sigma2 * t79 * t90;
        let t98 = piecewise3(t64, 0.0, 3.0 / 20.0 * t6 * t74 * t94);
        let tzk0 = t63 + t98;
        zk[ip] += tzk0;
    }
}
