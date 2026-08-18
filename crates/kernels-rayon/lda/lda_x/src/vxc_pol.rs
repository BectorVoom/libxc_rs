//! LDA_X vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_x_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    param_alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t8 = rho0 * t7;
        let t10 = 2.0 * t8 <= zeta_threshold;
        let t11 = pow_1_3(zeta_threshold);
        let t12 = t11 * zeta_threshold;
        let t13 = M_CBRT2;
        let t14 = t13 * rho0;
        let t15 = pow_1_3(t8);
        let t19 = piecewise3(t10, t12, 2.0 * t14 * t7 * t15);
        let t20 = pow_1_3(t6);
        let t24 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t19 * t20);
        let t25 = param_alpha * t24;
        let t26 = rho1 <= dens_threshold;
        let t27 = rho1 * t7;
        let t29 = 2.0 * t27 <= zeta_threshold;
        let t30 = t13 * rho1;
        let t31 = pow_1_3(t27);
        let t35 = piecewise3(t29, t12, 2.0 * t30 * t7 * t31);
        let t39 = piecewise3(t26, 0.0, -3.0 / 8.0 * t5 * t35 * t20);
        let t40 = param_alpha * t39;
        let tzk0 = t25 + t40;
        zk[ip] += tzk0;
        let t41 = t13 * t7;
        let t44 = t6 * t6;
        let t45 = 1.0 / t44;
        let t48 = 2.0 * t14 * t45 * t15;
        let t49 = t15 * t15;
        let t50 = 1.0 / t49;
        let t51 = t7 * t50;
        let t53 = -rho0 * t45 + t7;
        let t58 = piecewise3(t10, 0.0, 2.0 * t41 * t15 - t48 + 2.0 / 3.0 * t14 * t51 * t53);
        let t62 = t20 * t20;
        let t63 = 1.0 / t62;
        let t66 = t5 * t19 * t63 / 8.0;
        let t68 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t58 * t20 - t66);
        let t69 = param_alpha * t68;
        let t72 = 2.0 * t30 * t45 * t31;
        let t73 = rho1 * rho1;
        let t74 = t13 * t73;
        let t75 = t44 * t6;
        let t76 = 1.0 / t75;
        let t77 = t31 * t31;
        let t78 = 1.0 / t77;
        let t79 = t76 * t78;
        let t83 = piecewise3(t29, 0.0, -t72 - 2.0 / 3.0 * t74 * t79);
        let t89 = t5 * t35 * t63 / 8.0;
        let t91 = piecewise3(t26, 0.0, -3.0 / 8.0 * t5 * t83 * t20 - t89);
        let t92 = param_alpha * t91;
        let tvrho0 = t25 + t40 + t6 * (t69 + t92);
        vrho[ip * 2] += tvrho0;
        let t95 = rho0 * rho0;
        let t96 = t13 * t95;
        let t97 = t76 * t50;
        let t101 = piecewise3(t10, 0.0, -t48 - 2.0 / 3.0 * t96 * t97);
        let t106 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t101 * t20 - t66);
        let t107 = param_alpha * t106;
        let t110 = t7 * t78;
        let t112 = -rho1 * t45 + t7;
        let t117 = piecewise3(t29, 0.0, 2.0 * t41 * t31 - t72 + 2.0 / 3.0 * t30 * t110 * t112);
        let t122 = piecewise3(t26, 0.0, -3.0 / 8.0 * t5 * t117 * t20 - t89);
        let t123 = param_alpha * t122;
        let tvrho1 = t25 + t40 + t6 * (t107 + t123);
        vrho[ip * 2 + 1] += tvrho1;
    }
}
