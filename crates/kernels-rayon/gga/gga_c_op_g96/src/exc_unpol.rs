//! GGA_C_OP_G96 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_g96.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_op_g96_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = 1.0 <= zeta_threshold;
        let t4 = t1 || rho[ip] / 2.0 <= dens_threshold;
        let t5 = zeta_threshold - 1.0;
        let t6 = -t5;
        let t7 = piecewise5(t1, t5, t1, t6, 0.0);
        let t8 = t7 * t7;
        let t9 = 1.0 - t8;
        let t10 = t9 * rho[ip];
        let t11 = 1.0 + t7;
        let t14 = t11 * rho[ip] / 2.0 <= dens_threshold;
        let t15 = M_CBRT3;
        let t16 = t15 * t15;
        let t18 = pow_1_3(1.0 / M_PI);
        let t19 = 1.0 / t18;
        let t20 = t16 * t19;
        let t21 = M_CBRT4;
        let t22 = t20 * t21;
        let t23 = M_CBRT2;
        let t24 = t11 <= zeta_threshold;
        let t25 = 1.0 - t7;
        let t26 = t25 <= zeta_threshold;
        let t27 = piecewise5(t24, t5, t26, t6, t7);
        let t28 = 1.0 + t27;
        let t29 = t28 * rho[ip];
        let t30 = pow_1_3(t29);
        let t31 = 1.0 / t30;
        let t33 = rmath::sqrt(sigma[ip]);
        let t34 = t33 * t23;
        let t35 = pow_1_3(rho[ip]);
        let t37 = 1.0 / t35 / rho[ip];
        let t38 = t34 * t37;
        let t39 = rmath::sqrt(t38);
        let t40 = t39 * t38;
        let t44 = 1.0 + 2.0 / 1233.0 * t20 * t21 * t40;
        let t45 = 1.0 / t44;
        let t49 = piecewise3(t14, 0.0, t22 * t23 * t31 * t45 / 9.0);
        let t53 = t25 * rho[ip] / 2.0 <= dens_threshold;
        let t54 = piecewise5(t26, t5, t24, t6, -t7);
        let t55 = 1.0 + t54;
        let t56 = t55 * rho[ip];
        let t57 = pow_1_3(t56);
        let t58 = 1.0 / t57;
        let t63 = piecewise3(t53, 0.0, t22 * t23 * t58 * t45 / 9.0);
        let t64 = t49 + t63;
        let t65 = t64 == 0.0;
        let t66 = piecewise3(t65, f64::EPSILON, t64);
        let t69 = 3.59628532 / t66 + 0.5764;
        let t70 = t66 * t66;
        let t71 = t70 * t70;
        let t72 = 1.0 / t71;
        let t74 = t70 * t66;
        let t75 = 1.0 / t74;
        let t77 = 1.0 / t70;
        let t79 = 31.220719919544194 * t72 + 14.903739892213245 * t75 + 1.778517305052 * t77;
        let t80 = 1.0 / t79;
        let tzk0 = piecewise3(t4, 0.0, -0.25 * t10 * t69 * t80);
        zk[ip] += tzk0;
    }
}
