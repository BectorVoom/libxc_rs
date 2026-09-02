//! GGA_C_OP_XALPHA exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_xalpha.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_op_xalpha_exc_pol(
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
        let t1 = rho0 - rho1;
        let t2 = rho0 + rho1;
        let t3 = 1.0 / t2;
        let t4 = t1 * t3;
        let t5 = rmath::abs(t4);
        let t11 = 1.0 - t5 <= zeta_threshold || rho0 <= dens_threshold && rho1 <= dens_threshold;
        let t13 = 1.0 + t4 <= zeta_threshold;
        let t14 = zeta_threshold - 1.0;
        let t16 = 1.0 - t4 <= zeta_threshold;
        let t17 = -t14;
        let t18 = piecewise5(t13, t14, t16, t17, t4);
        let t19 = t18 * t18;
        let t20 = 1.0 - t19;
        let t21 = t20 * t2;
        let t24 = 2.0 * rho0 * t3 <= zeta_threshold;
        let t27 = 2.0 * rho1 * t3 <= zeta_threshold;
        let t28 = piecewise5(t24, t14, t27, t17, t4);
        let t29 = 1.0 + t28;
        let t32 = t29 * t2 / 2.0 <= dens_threshold;
        let t33 = M_CBRT3;
        let t34 = t33 * t33;
        let t36 = pow_1_3(1.0 / M_PI);
        let t38 = t34 / t36;
        let t39 = M_CBRT4;
        let t40 = M_CBRT2;
        let t41 = t39 * t40;
        let t42 = t29 <= zeta_threshold;
        let t43 = 1.0 - t28;
        let t44 = t43 <= zeta_threshold;
        let t45 = piecewise5(t42, t14, t44, t17, t28);
        let t46 = 1.0 + t45;
        let t47 = t46 * t2;
        let t48 = pow_1_3(t47);
        let t53 = piecewise3(t32, 0.0, t38 * t41 / t48 / 9.0);
        let t57 = t43 * t2 / 2.0 <= dens_threshold;
        let t58 = piecewise5(t44, t14, t42, t17, -t28);
        let t59 = 1.0 + t58;
        let t60 = t59 * t2;
        let t61 = pow_1_3(t60);
        let t66 = piecewise3(t57, 0.0, t38 * t41 / t61 / 9.0);
        let t67 = t53 + t66;
        let t68 = t67 == 0.0;
        let t69 = piecewise3(t68, f64::EPSILON, t67);
        let t72 = 3.90299956 / t69 + 0.5764;
        let t73 = t69 * t69;
        let t74 = t73 * t73;
        let t75 = 1.0 / t74;
        let t77 = t73 * t69;
        let t78 = 1.0 / t77;
        let t80 = 1.0 / t73;
        let t82 = 43.31320905673766 * t75 + 19.051463748196298 * t78 + 2.094820520028 * t80;
        let t83 = 1.0 / t82;
        let t84 = t72 * t83;
        let tzk0 = piecewise3(t11, 0.0, -0.25 * t21 * t84);
        zk[ip] += tzk0;
    }
}
