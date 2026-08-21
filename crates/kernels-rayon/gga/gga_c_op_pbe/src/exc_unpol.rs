//! GGA_C_OP_PBE exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_pbe.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_op_pbe_exc_unpol(
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
        let t20 = t16 / t18;
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
        let t33 = M_CBRT6;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = 1.0 / t36;
        let t38 = t33 * t37;
        let t39 = t23 * t23;
        let t41 = rho[ip] * rho[ip];
        let t42 = pow_1_3(rho[ip]);
        let t43 = t42 * t42;
        let t45 = 1.0 / t43 / t41;
        let t49 = 0.804 + 0.009146457198521547 * t38 * sigma[ip] * t39 * t45;
        let t52 = 1.804 - 0.646416 / t49;
        let t53 = 1.0 / t52;
        let t57 = piecewise3(t14, 0.0, t22 * t23 * t31 * t53 / 9.0);
        let t61 = t25 * rho[ip] / 2.0 <= dens_threshold;
        let t62 = piecewise5(t26, t5, t24, t6, -t7);
        let t63 = 1.0 + t62;
        let t64 = t63 * rho[ip];
        let t65 = pow_1_3(t64);
        let t66 = 1.0 / t65;
        let t71 = piecewise3(t61, 0.0, t22 * t23 * t66 * t53 / 9.0);
        let t72 = t57 + t71;
        let t73 = t72 == 0.0;
        let t74 = piecewise3(t73, f64::EPSILON, t72);
        let t77 = 3.61925846 / t74 + 0.5764;
        let t78 = t74 * t74;
        let t79 = t78 * t78;
        let t80 = 1.0 / t79;
        let t82 = t78 * t74;
        let t83 = 1.0 / t82;
        let t85 = 1.0 / t78;
        let t87 = 32.02615087407435 * t80 + 15.19118443242906 * t83 + 1.801312286343 * t85;
        let t88 = 1.0 / t87;
        let tzk0 = piecewise3(t4, 0.0, -0.25 * t10 * t77 * t88);
        zk[ip] += tzk0;
    }
}
