//! MGGA_X_BR89 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_br89.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::br89::{xc_mgga_x_br89_get_x};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_br89_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_at: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = 1.0 <= zeta_threshold;
        let t5 = zeta_threshold - 1.0;
        let t7 = piecewise5(t4, t5, t4, -t5, 0.0);
        let t8 = 1.0 + t7;
        let t10 = pow_1_3(zeta_threshold);
        let t12 = pow_1_3(t8);
        let t14 = piecewise3(t8 <= zeta_threshold, t10 * zeta_threshold, t12 * t8);
        let t15 = pow_1_3(rho[ip]);
        let t16 = t14 * t15;
        let t18 = pow_1_3(1.0 / M_PI);
        let t19 = 1.0 / t18;
        let t20 = M_CBRT4;
        let t21 = t19 * t20;
        let t22 = t16 * t21;
        let t23 = M_CBRT2;
        let t24 = t23 * t23;
        let t25 = t15 * t15;
        let t27 = 1.0 / t25 / rho[ip];
        let t30 = param_gamma * tau[ip];
        let t33 = param_gamma * sigma[ip];
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t25 / t34;
        let t40 = rmath::abs(lapl[ip] * t27 / 2.0 - 2.0 * t30 * t27 + t33 * t36 / 4.0);
        let t43 = t24 * t40 / 3.0 < 5e-13;
        let t44 = lapl[ip] * t24;
        let t47 = t24 * t27;
        let t50 = t24 * t36;
        let t53 = t44 * t27 / 6.0 - 2.0 / 3.0 * t30 * t47 + t33 * t50 / 12.0;
        let t54 = 0.0 < t53;
        let t55 = piecewise3(t54, 5e-13, -5e-13);
        let t56 = piecewise3(t43, t55, t53);
        let t57 = xc_mgga_x_br89_get_x(t56);
        let t59 = rmath::exp(t57 / 3.0);
        let t60 = rmath::exp(-t57);
        let t62 = 1.0 + t57 / 2.0;
        let t63 = t60 * t62;
        let t64 = 1.0 - t63;
        let t65 = t59 * t64;
        let t66 = 1.0 / t57;
        let t67 = M_CBRT6;
        let t68 = t67 * t67;
        let t69 = M_PI * M_PI;
        let t70 = pow_1_3(t69);
        let t71 = t70 * t70;
        let t73 = 3.0 / 10.0 * t68 * t71;
        let t74 = tau[ip] * t24;
        let t75 = t74 * t27;
        let t76 = t73 - t75;
        let t77 = t73 + t75;
        let t78 = 1.0 / t77;
        let t80 = t76 * t76;
        let t81 = t80 * t76;
        let t82 = t77 * t77;
        let t83 = t82 * t77;
        let t84 = 1.0 / t83;
        let t87 = t80 * t80;
        let t88 = t87 * t76;
        let t89 = t82 * t82;
        let t91 = 1.0 / t89 / t77;
        let t95 = 1.0 + param_at * (t76 * t78 - 2.0 * t81 * t84 + t88 * t91);
        let t96 = t66 * t95;
        let t97 = t65 * t96;
        let t100 = piecewise3(t3, 0.0, -t22 * t97 / 4.0);
        let tzk0 = 2.0 * t100;
        zk[ip] += tzk0;
    }
}
