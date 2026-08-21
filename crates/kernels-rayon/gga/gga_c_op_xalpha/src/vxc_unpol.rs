//! GGA_C_OP_XALPHA vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_xalpha.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_op_xalpha_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let t22 = M_CBRT2;
        let t23 = t21 * t22;
        let t24 = t11 <= zeta_threshold;
        let t25 = 1.0 - t7;
        let t26 = t25 <= zeta_threshold;
        let t27 = piecewise5(t24, t5, t26, t6, t7);
        let t28 = 1.0 + t27;
        let t29 = t28 * rho[ip];
        let t30 = pow_1_3(t29);
        let t35 = piecewise3(t14, 0.0, t20 * t23 / t30 / 9.0);
        let t39 = t25 * rho[ip] / 2.0 <= dens_threshold;
        let t40 = piecewise5(t26, t5, t24, t6, -t7);
        let t41 = 1.0 + t40;
        let t42 = t41 * rho[ip];
        let t43 = pow_1_3(t42);
        let t48 = piecewise3(t39, 0.0, t20 * t23 / t43 / 9.0);
        let t49 = t35 + t48;
        let t50 = t49 == 0.0;
        let t51 = piecewise3(t50, f64::EPSILON, t49);
        let t54 = 3.90299956 / t51 + 0.5764;
        let t55 = t51 * t51;
        let t56 = t55 * t55;
        let t57 = 1.0 / t56;
        let t59 = t55 * t51;
        let t60 = 1.0 / t59;
        let t62 = 1.0 / t55;
        let t64 = 43.31320905673766 * t57 + 19.051463748196298 * t60 + 2.094820520028 * t62;
        let t65 = 1.0 / t64;
        let tzk0 = piecewise3(t4, 0.0, -0.25 * t10 * t54 * t65);
        zk[ip] += tzk0;
        let t69 = t9 * t54;
        let t72 = t20 * t21;
        let t79 = piecewise3(t14, 0.0, -t72 * t22 / t30 / t29 * t28 / 27.0);
        let t86 = piecewise3(t39, 0.0, -t72 * t22 / t43 / t42 * t41 / 27.0);
        let t88 = piecewise3(t50, 0.0, t79 + t86);
        let t93 = t64 * t64;
        let t94 = 1.0 / t93;
        let t95 = t54 * t94;
        let t97 = 1.0 / t56 / t51;
        let t98 = t97 * t88;
        let t100 = t57 * t88;
        let t104 = -173.25283622695065 * t98 - 57.15439124458889 * t100 - 4.189641040056 * t60 * t88;
        let t109 = piecewise3(t4, 0.0, -0.25 * t69 * t65 + 0.97574989 * t10 * t62 * t88 * t65 + 0.25 * t10 * t95 * t104);
        let tvrho0 = rho[ip] * t109 + tzk0;
        vrho[ip] += tvrho0;
        let tvsigma0 = 0.0;
        vsigma[ip] += tvsigma0;
    }
}
