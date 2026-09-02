//! LDA_X_SLOC fxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_sloc.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_x_sloc_fxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = param_b + 1.0;
        let t3 = 1.0 / t1 / 2.0;
        let t4 = param_a * t3;
        let t5 = rho0 + rho1;
        let t6 = rmath::pow(t5, param_b);
        let t7 = rho0 - rho1;
        let t8 = 1.0 / t5;
        let t9 = t7 * t8;
        let t10 = 1.0 + t9;
        let t11 = t10 <= zeta_threshold;
        let t12 = rmath::pow(zeta_threshold, t1);
        let t13 = rmath::pow(t10, t1);
        let t14 = piecewise3(t11, t12, t13);
        let t15 = 1.0 - t9;
        let t16 = t15 <= zeta_threshold;
        let t17 = rmath::pow(t15, t1);
        let t18 = piecewise3(t16, t12, t17);
        let t19 = t14 + t18;
        let tzk0 = -t4 * t6 * t19;
        zk[ip] += tzk0;
        let t22 = t6 * param_b;
        let t24 = t4 * t22 * t19;
        let t25 = t5 * param_a;
        let t26 = t3 * t6;
        let t27 = t13 * t1;
        let t28 = t5 * t5;
        let t29 = 1.0 / t28;
        let t30 = t7 * t29;
        let t31 = t8 - t30;
        let t32 = 1.0 / t10;
        let t35 = piecewise3(t11, 0.0, t27 * t31 * t32);
        let t36 = t17 * t1;
        let t37 = -t31;
        let t38 = 1.0 / t15;
        let t41 = piecewise3(t16, 0.0, t36 * t37 * t38);
        let t42 = t35 + t41;
        let tvrho0 = -t25 * t26 * t42 - t24 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t45 = -t8 - t30;
        let t48 = piecewise3(t11, 0.0, t27 * t45 * t32);
        let t49 = -t45;
        let t52 = piecewise3(t16, 0.0, t36 * t49 * t38);
        let t53 = t48 + t52;
        let tvrho1 = -t25 * t26 * t53 - t24 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
        let t56 = t4 * t6;
        let t57 = param_b * t8;
        let t59 = t56 * t57 * t19;
        let t61 = t4 * t6 * t42;
        let t63 = param_b * param_b;
        let t64 = t63 * t8;
        let t66 = t56 * t64 * t19;
        let t68 = t4 * t22 * t42;
        let t70 = t1 * t1;
        let t71 = t13 * t70;
        let t72 = t31 * t31;
        let t73 = t10 * t10;
        let t74 = 1.0 / t73;
        let t75 = t72 * t74;
        let t78 = 1.0 / t28 / t5;
        let t79 = t7 * t78;
        let t81 = -2.0 * t29 + 2.0 * t79;
        let t86 = piecewise3(t11, 0.0, t27 * t81 * t32 - t27 * t75 + t71 * t75);
        let t87 = t17 * t70;
        let t88 = t37 * t37;
        let t89 = t15 * t15;
        let t90 = 1.0 / t89;
        let t91 = t88 * t90;
        let t93 = -t81;
        let t98 = piecewise3(t16, 0.0, t36 * t93 * t38 - t36 * t91 + t87 * t91);
        let t99 = t86 + t98;
        let tv2rho20 = -t25 * t26 * t99 - t59 - 2.0 * t61 - t66 - 2.0 * t68;
        v2rho2[ip * 3] += tv2rho20;
        let t103 = t4 * t6 * t53;
        let t105 = t4 * t22 * t53;
        let t106 = t31 * t74;
        let t107 = t106 * t45;
        let t114 = piecewise3(t11, 0.0, 2.0 * t27 * t79 * t32 - t27 * t107 + t71 * t107);
        let t115 = t37 * t90;
        let t116 = t115 * t49;
        let t123 = piecewise3(t16, 0.0, -2.0 * t36 * t79 * t38 - t36 * t116 + t87 * t116);
        let t124 = t114 + t123;
        let tv2rho21 = -t25 * t26 * t124 - t103 - t105 - t59 - t61 - t66 - t68;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t129 = t45 * t45;
        let t130 = t129 * t74;
        let t133 = 2.0 * t29 + 2.0 * t79;
        let t138 = piecewise3(t11, 0.0, t27 * t133 * t32 - t27 * t130 + t71 * t130);
        let t139 = t49 * t49;
        let t140 = t139 * t90;
        let t142 = -t133;
        let t147 = piecewise3(t16, 0.0, t36 * t142 * t38 - t36 * t140 + t87 * t140);
        let t148 = t138 + t147;
        let tv2rho22 = -t25 * t26 * t148 - 2.0 * t103 - 2.0 * t105 - t59 - t66;
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}
