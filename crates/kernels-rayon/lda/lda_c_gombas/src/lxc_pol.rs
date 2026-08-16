//! LDA_C_GOMBAS lxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_gombas.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::powers::{pow_1_3};

/// LDA_C_GOMBAS lxc -- polarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_c_gombas_lxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3(t1);
        let t3 = 1.0 / t2;
        let t5 = 1.0 + 0.0562 * t3;
        let t7 = 0.0357 / t5;
        let t8 = t3 + 2.39;
        let t10 = f64::ln(t8 * t2);
        let t11 = 0.0311 * t10;
        let tzk0 = -t7 - t11;
        zk[ip] += tzk0;
        let t12 = t5 * t5;
        let t13 = 1.0 / t12;
        let t15 = 1.0 / t2 / t1;
        let t16 = t13 * t15;
        let t19 = t2 * t2;
        let t23 = -1.0 / t1 / 3.0 + t8 / t19 / 3.0;
        let t24 = 1.0 / t8;
        let t25 = t23 * t24;
        let t26 = t25 * t3;
        let tvrho0 = -t7 - t11 + t1 * (-0.00066878 * t16 - 0.0311 * t26);
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t33 = 1.0 / t12 / t5;
        let t34 = t1 * t1;
        let t36 = 1.0 / t19 / t34;
        let t37 = t33 * t36;
        let t40 = 1.0 / t2 / t34;
        let t41 = t13 * t40;
        let t45 = 1.0 / t19 / t1;
        let t48 = 2.0 / 9.0 / t34 - 2.0 / 9.0 * t8 * t45;
        let t49 = t48 * t24;
        let t50 = t49 * t3;
        let t52 = t8 * t8;
        let t53 = 1.0 / t52;
        let t54 = t23 * t53;
        let t55 = t54 * t45;
        let t57 = t25 * t15;
        let tv2rho20 = -0.00133756 * t16 - 0.0622 * t26 + t1 * (-2.5056957333333333e-05 * t37 + 0.0008917066666666667 * t41 - 0.0311 * t50 - 0.010366666666666666 * t55 + 0.010366666666666666 * t57);
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t66 = t12 * t12;
        let t67 = 1.0 / t66;
        let t68 = t34 * t34;
        let t69 = 1.0 / t68;
        let t70 = t67 * t69;
        let t72 = t34 * t1;
        let t74 = 1.0 / t19 / t72;
        let t75 = t33 * t74;
        let t78 = 1.0 / t2 / t72;
        let t79 = t13 * t78;
        let t81 = 1.0 / t72;
        let t84 = 10.0 / 27.0 * t8 * t36 - 10.0 / 27.0 * t81;
        let t85 = t84 * t24;
        let t86 = t85 * t3;
        let t88 = t48 * t53;
        let t89 = t88 * t45;
        let t91 = t49 * t15;
        let t94 = 1.0 / t52 / t8;
        let t95 = t23 * t94;
        let t96 = t95 * t81;
        let t98 = t54 * t36;
        let t100 = t25 * t40;
        let tv3rho30 = -7.5170872e-05 * t37 + 0.00267512 * t41 - 0.0933 * t50 - 0.0311 * t55 + 0.0311 * t57 + t1 * (-1.4082010021333333e-06 * t70 + 0.00010022782933333333 * t75 - 0.0020806488888888888 * t79 - 0.0311 * t86 - 0.020733333333333333 * t89 + 0.020733333333333333 * t91 - 0.006911111111111111 * t96 + 0.020733333333333333 * t98 - 0.013822222222222222 * t100);
        v3rho3[ip * 4] += tv3rho30;
        let tv3rho31 = tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let tv3rho32 = tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let tv3rho33 = tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
        let t115 = t68 * t1;
        let t128 = 1.0 / t2 / t68;
        let t149 = t52 * t52;
        let t160 = -1.0552119509319111e-07 / t66 / t5 / t2 / t115 + 1.1265608017066667e-05 * t67 / t115 - 0.00044545701925925924 * t33 / t19 / t68 + 0.006935496296296297 * t13 * t128 - 0.0311 * (-80.0 / 81.0 * t8 * t74 + 80.0 / 81.0 * t69) * t24 * t3 - 0.0311 * t84 * t53 * t45 + 0.0311 * t85 * t15 - 0.020733333333333333 * t48 * t94 * t81 + 0.0622 * t88 * t36 - 0.041466666666666666 * t49 * t40 - 0.006911111111111111 * t23 / t149 * t128 + 0.034555555555555555 * t95 * t69 - 0.059896296296296295 * t54 * t74 + 0.03225185185185185 * t25 * t78;
        let tv4rho40 = -5.632804008533333e-06 * t70 + 0.00040091131733333333 * t75 - 0.008322595555555555 * t79 - 0.1244 * t86 - 0.08293333333333333 * t89 + 0.08293333333333333 * t91 - 0.027644444444444444 * t96 + 0.08293333333333333 * t98 - 0.05528888888888889 * t100 + t1 * t160;
        v4rho4[ip * 5] += tv4rho40;
        let tv4rho41 = tv4rho40;
        v4rho4[ip * 5 + 1] += tv4rho41;
        let tv4rho42 = tv4rho41;
        v4rho4[ip * 5 + 2] += tv4rho42;
        let tv4rho43 = tv4rho42;
        v4rho4[ip * 5 + 3] += tv4rho43;
        let tv4rho44 = tv4rho43;
        v4rho4[ip * 5 + 4] += tv4rho44;
    }
}
