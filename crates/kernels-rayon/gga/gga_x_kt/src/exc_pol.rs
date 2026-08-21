//! GGA_X_KT exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_kt.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_kt_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_gamma: f64,
    param_delta: f64,
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
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
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
        let t28 = t2 * t2;
        let t29 = param_gamma * t28;
        let t31 = pow_1_3(1.0 / M_PI);
        let t32 = 1.0 / t31;
        let t33 = M_CBRT4;
        let t34 = t32 * t33;
        let t35 = t29 * t34;
        let t36 = M_CBRT2;
        let t37 = t36 * t36;
        let t38 = t19 * t6;
        let t39 = pow_1_3(t38);
        let t40 = t39 * t38;
        let t41 = t37 * t40;
        let t42 = rho0 * rho0;
        let t43 = pow_1_3(rho0);
        let t44 = t43 * t43;
        let t46 = 1.0 / t44 / t42;
        let t47 = sigma0 * t46;
        let t49 = t41 / 4.0 + param_delta;
        let t50 = 1.0 / t49;
        let t51 = t47 * t50;
        let t55 = 1.0 - t35 * t41 * t51 / 18.0;
        let t59 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t55);
        let t60 = rho1 <= dens_threshold;
        let t61 = -t16;
        let t63 = piecewise5(t14, t11, t10, t15, t61 * t7);
        let t64 = 1.0 + t63;
        let t65 = t64 <= zeta_threshold;
        let t66 = pow_1_3(t64);
        let t68 = piecewise3(t65, t22, t66 * t64);
        let t69 = t68 * t26;
        let t70 = t64 * t6;
        let t71 = pow_1_3(t70);
        let t72 = t71 * t70;
        let t73 = t37 * t72;
        let t74 = rho1 * rho1;
        let t75 = pow_1_3(rho1);
        let t76 = t75 * t75;
        let t78 = 1.0 / t76 / t74;
        let t79 = sigma2 * t78;
        let t81 = t73 / 4.0 + param_delta;
        let t82 = 1.0 / t81;
        let t83 = t79 * t82;
        let t87 = 1.0 - t35 * t73 * t83 / 18.0;
        let t91 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t69 * t87);
        let tzk0 = t59 + t91;
        zk[ip] += tzk0;
    }
}
