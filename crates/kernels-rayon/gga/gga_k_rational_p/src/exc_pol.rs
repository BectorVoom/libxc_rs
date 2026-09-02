//! GGA_K_RATIONAL_P exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_rational_p.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_rational_p_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_p: f64,
    param_C2: f64,
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
        let t32 = 1.0 / param_p;
        let t34 = M_CBRT6;
        let t35 = param_C2 * t32 * t34;
        let t36 = M_PI * M_PI;
        let t37 = pow_1_3(t36);
        let t38 = t37 * t37;
        let t39 = 1.0 / t38;
        let t41 = rho0 * rho0;
        let t42 = pow_1_3(rho0);
        let t43 = t42 * t42;
        let t45 = 1.0 / t43 / t41;
        let t49 = 1.0 + t35 * t39 * sigma0 * t45 / 24.0;
        let t50 = rmath::pow(t49, -param_p);
        let t51 = t31 * t50;
        let t52 = t6 * t51;
        let t54 = piecewise3(t1, 0.0, 3.0 / 20.0 * t52);
        let t55 = rho1 <= dens_threshold;
        let t56 = -t17;
        let t58 = piecewise5(t15, t12, t11, t16, t56 * t8);
        let t59 = 1.0 + t58;
        let t60 = t59 <= zeta_threshold;
        let t61 = pow_1_3(t59);
        let t62 = t61 * t61;
        let t64 = piecewise3(t60, t24, t62 * t59);
        let t65 = t64 * t30;
        let t67 = rho1 * rho1;
        let t68 = pow_1_3(rho1);
        let t69 = t68 * t68;
        let t71 = 1.0 / t69 / t67;
        let t75 = 1.0 + t35 * t39 * sigma2 * t71 / 24.0;
        let t76 = rmath::pow(t75, -param_p);
        let t77 = t65 * t76;
        let t78 = t6 * t77;
        let t80 = piecewise3(t55, 0.0, 3.0 / 20.0 * t78);
        let tzk0 = t54 + t80;
        zk[ip] += tzk0;
    }
}
