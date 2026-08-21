//! LDA_K_TF fxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_k_tf.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_k_tf_fxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    param_ax: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 - rho1;
        let t2 = rho0 + rho1;
        let t3 = 1.0 / t2;
        let t4 = t1 * t3;
        let t5 = 1.0 + t4;
        let t6 = t5 <= zeta_threshold;
        let t7 = pow_1_3(zeta_threshold);
        let t8 = t7 * t7;
        let t9 = t8 * zeta_threshold;
        let t10 = pow_1_3(t5);
        let t11 = t10 * t10;
        let t13 = piecewise3(t6, t9, t11 * t5);
        let t14 = 1.0 - t4;
        let t15 = t14 <= zeta_threshold;
        let t16 = pow_1_3(t14);
        let t17 = t16 * t16;
        let t19 = piecewise3(t15, t9, t17 * t14);
        let t23 = M_CBRT3;
        let t24 = param_ax * (t13 / 2.0 + t19 / 2.0) * t23;
        let t26 = pow_1_3(1.0 / M_PI);
        let t27 = t26 * t26;
        let t28 = 1.0 / t27;
        let t29 = M_CBRT4;
        let t30 = t29 * t29;
        let t31 = t28 * t30;
        let t32 = pow_1_3(t2);
        let t33 = t32 * t32;
        let t34 = t31 * t33;
        let t35 = t24 * t34;
        let tzk0 = t35 / 3.0;
        zk[ip] += tzk0;
        let t36 = 5.0 / 9.0 * t35;
        let t38 = t33 * t2 * param_ax;
        let t39 = t2 * t2;
        let t40 = 1.0 / t39;
        let t41 = t1 * t40;
        let t42 = t3 - t41;
        let t45 = piecewise3(t6, 0.0, 5.0 / 3.0 * t11 * t42);
        let t46 = -t42;
        let t49 = piecewise3(t15, 0.0, 5.0 / 3.0 * t17 * t46);
        let t51 = t45 / 2.0 + t49 / 2.0;
        let t54 = t23 * t28 * t30;
        let tvrho0 = t36 + t38 * t51 * t54 / 3.0;
        vrho[ip * 2] += tvrho0;
        let t57 = -t3 - t41;
        let t60 = piecewise3(t6, 0.0, 5.0 / 3.0 * t11 * t57);
        let t61 = -t57;
        let t64 = piecewise3(t15, 0.0, 5.0 / 3.0 * t17 * t61);
        let t66 = t60 / 2.0 + t64 / 2.0;
        let tvrho1 = t36 + t38 * t66 * t54 / 3.0;
        vrho[ip * 2 + 1] += tvrho1;
        let t71 = param_ax * t51 * t23;
        let t72 = t71 * t34;
        let t74 = 1.0 / t32;
        let t75 = t31 * t74;
        let t77 = 10.0 / 27.0 * t24 * t75;
        let t78 = 1.0 / t10;
        let t79 = t42 * t42;
        let t83 = 1.0 / t39 / t2;
        let t84 = t1 * t83;
        let t86 = -2.0 * t40 + 2.0 * t84;
        let t90 = piecewise3(t6, 0.0, 10.0 / 9.0 * t78 * t79 + 5.0 / 3.0 * t11 * t86);
        let t91 = 1.0 / t16;
        let t92 = t46 * t46;
        let t95 = -t86;
        let t99 = piecewise3(t15, 0.0, 10.0 / 9.0 * t91 * t92 + 5.0 / 3.0 * t17 * t95);
        let t101 = t90 / 2.0 + t99 / 2.0;
        let tv2rho20 = 10.0 / 9.0 * t72 + t77 + t38 * t101 * t54 / 3.0;
        v2rho2[ip * 3] += tv2rho20;
        let t106 = t33 * param_ax;
        let t108 = t106 * t66 * t54;
        let t110 = t78 * t57;
        let t113 = t11 * t1;
        let t117 = piecewise3(t6, 0.0, 10.0 / 9.0 * t110 * t42 + 10.0 / 3.0 * t113 * t83);
        let t118 = t91 * t61;
        let t121 = t17 * t1;
        let t125 = piecewise3(t15, 0.0, 10.0 / 9.0 * t118 * t46 - 10.0 / 3.0 * t121 * t83);
        let t127 = t117 / 2.0 + t125 / 2.0;
        let tv2rho21 = 5.0 / 9.0 * t72 + t77 + 5.0 / 9.0 * t108 + t38 * t127 * t54 / 3.0;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t132 = t57 * t57;
        let t136 = 2.0 * t40 + 2.0 * t84;
        let t140 = piecewise3(t6, 0.0, 10.0 / 9.0 * t78 * t132 + 5.0 / 3.0 * t11 * t136);
        let t141 = t61 * t61;
        let t144 = -t136;
        let t148 = piecewise3(t15, 0.0, 10.0 / 9.0 * t91 * t141 + 5.0 / 3.0 * t17 * t144);
        let t150 = t140 / 2.0 + t148 / 2.0;
        let tv2rho22 = 10.0 / 9.0 * t108 + t77 + t38 * t150 * t54 / 3.0;
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}
