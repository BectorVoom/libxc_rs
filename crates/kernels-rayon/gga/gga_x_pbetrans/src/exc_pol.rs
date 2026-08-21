//! GGA_X_PBETRANS exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbetrans.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_pbetrans_exc_pol(
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
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_PI * M_PI;
        let t29 = pow_1_3(t28);
        let t30 = t2 * t29;
        let t31 = M_CBRT6;
        let t32 = t31 * t31;
        let t34 = t32 / t29;
        let t35 = rmath::sqrt(sigma0);
        let t36 = pow_1_3(rho0);
        let t38 = 1.0 / t36 / rho0;
        let t45 = rmath::exp(-2.0 * t30 * (t34 * t35 * t38 / 12.0 - 3.0));
        let t46 = 1.0 + t45;
        let t48 = 0.413 / t46;
        let t49 = 1.227 - t48;
        let t50 = t29 * t29;
        let t52 = t31 / t50;
        let t53 = rho0 * rho0;
        let t54 = t36 * t36;
        let t56 = 1.0 / t54 / t53;
        let t60 = 1.227 - t48 + 0.009125 * t52 * sigma0 * t56;
        let t61 = 1.0 / t60;
        let t63 = -t49 * t61 + 1.0;
        let t65 = t49 * t63 + 1.0;
        let t69 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t65);
        let t70 = rho1 <= dens_threshold;
        let t71 = -t16;
        let t73 = piecewise5(t14, t11, t10, t15, t71 * t7);
        let t74 = 1.0 + t73;
        let t75 = t74 <= zeta_threshold;
        let t76 = pow_1_3(t74);
        let t78 = piecewise3(t75, t22, t76 * t74);
        let t79 = t78 * t26;
        let t80 = rmath::sqrt(sigma2);
        let t81 = pow_1_3(rho1);
        let t83 = 1.0 / t81 / rho1;
        let t90 = rmath::exp(-2.0 * t30 * (t34 * t80 * t83 / 12.0 - 3.0));
        let t91 = 1.0 + t90;
        let t93 = 0.413 / t91;
        let t94 = 1.227 - t93;
        let t95 = rho1 * rho1;
        let t96 = t81 * t81;
        let t98 = 1.0 / t96 / t95;
        let t102 = 1.227 - t93 + 0.009125 * t52 * sigma2 * t98;
        let t103 = 1.0 / t102;
        let t105 = -t94 * t103 + 1.0;
        let t107 = t94 * t105 + 1.0;
        let t111 = piecewise3(t70, 0.0, -3.0 / 8.0 * t5 * t79 * t107);
        let tzk0 = t69 + t111;
        zk[ip] += tzk0;
    }
}
