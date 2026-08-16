//! LDA_K_TF kxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_k_tf.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3};
use libxc_rkernel_math::piecewise::{piecewise3};

/// LDA_K_TF kxc -- polarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_k_tf_kxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
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
        let t155 = param_ax * t101 * t23;
        let t156 = t155 * t34;
        let t158 = t71 * t75;
        let t161 = 1.0 / t32 / t2;
        let t162 = t31 * t161;
        let t164 = 10.0 / 81.0 * t24 * t162;
        let t166 = 1.0 / t10 / t5;
        let t167 = t79 * t42;
        let t170 = t78 * t42;
        let t173 = t39 * t39;
        let t174 = 1.0 / t173;
        let t175 = t1 * t174;
        let t177 = 6.0 * t83 - 6.0 * t175;
        let t181 = piecewise3(t6, 0.0, -10.0 / 27.0 * t166 * t167 + 10.0 / 3.0 * t170 * t86 + 5.0 / 3.0 * t11 * t177);
        let t183 = 1.0 / t16 / t14;
        let t184 = t92 * t46;
        let t187 = t91 * t46;
        let t190 = -t177;
        let t194 = piecewise3(t15, 0.0, -10.0 / 27.0 * t183 * t184 + 10.0 / 3.0 * t187 * t95 + 5.0 / 3.0 * t17 * t190);
        let t196 = t181 / 2.0 + t194 / 2.0;
        let tv3rho30 = 5.0 / 3.0 * t156 + 10.0 / 9.0 * t158 - t164 + t38 * t196 * t54 / 3.0;
        v3rho3[ip * 4] += tv3rho30;
        let t202 = t74 * param_ax;
        let t204 = t202 * t66 * t54;
        let t208 = 10.0 / 9.0 * t106 * t127 * t54;
        let t209 = t166 * t57;
        let t212 = t78 * t1;
        let t223 = piecewise3(t6, 0.0, -10.0 / 27.0 * t209 * t79 + 40.0 / 9.0 * t212 * t83 * t42 + 10.0 / 9.0 * t110 * t86 + 10.0 / 3.0 * t11 * t83 - 10.0 * t113 * t174);
        let t224 = t183 * t61;
        let t227 = t91 * t1;
        let t238 = piecewise3(t15, 0.0, -10.0 / 27.0 * t224 * t92 - 40.0 / 9.0 * t227 * t83 * t46 + 10.0 / 9.0 * t118 * t95 - 10.0 / 3.0 * t17 * t83 + 10.0 * t121 * t174);
        let t240 = t223 / 2.0 + t238 / 2.0;
        let tv3rho31 = 5.0 / 9.0 * t156 + 20.0 / 27.0 * t158 - t164 + 10.0 / 27.0 * t204 + t208 + t38 * t240 * t54 / 3.0;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let t247 = t106 * t150 * t54;
        let t249 = t166 * t132;
        let t254 = t78 * t136;
        let t259 = -2.0 * t83 - 6.0 * t175;
        let t263 = piecewise3(t6, 0.0, -10.0 / 27.0 * t249 * t42 + 40.0 / 9.0 * t110 * t84 + 10.0 / 9.0 * t254 * t42 + 5.0 / 3.0 * t11 * t259);
        let t264 = t183 * t141;
        let t269 = t91 * t144;
        let t272 = -t259;
        let t276 = piecewise3(t15, 0.0, -10.0 / 27.0 * t264 * t46 - 40.0 / 9.0 * t118 * t84 + 10.0 / 9.0 * t269 * t46 + 5.0 / 3.0 * t17 * t272);
        let t278 = t263 / 2.0 + t276 / 2.0;
        let tv3rho32 = 20.0 / 27.0 * t204 + t208 + 10.0 / 27.0 * t158 - t164 + 5.0 / 9.0 * t247 + t38 * t278 * t54 / 3.0;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let t284 = t132 * t57;
        let t290 = -6.0 * t83 - 6.0 * t175;
        let t294 = piecewise3(t6, 0.0, -10.0 / 27.0 * t166 * t284 + 10.0 / 3.0 * t110 * t136 + 5.0 / 3.0 * t11 * t290);
        let t295 = t141 * t61;
        let t300 = -t290;
        let t304 = piecewise3(t15, 0.0, -10.0 / 27.0 * t183 * t295 + 10.0 / 3.0 * t118 * t144 + 5.0 / 3.0 * t17 * t300);
        let t306 = t294 / 2.0 + t304 / 2.0;
        let tv3rho33 = 10.0 / 9.0 * t204 + 5.0 / 3.0 * t247 - t164 + t38 * t306 * t54 / 3.0;
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}
