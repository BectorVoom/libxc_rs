//! LDA_C_VWN_3 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_3.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_vwn_3_exc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t8 = 1.0 / t7;
        let t9 = t6 * t8;
        let t10 = t4 * t9;
        let t11 = t10 / 4.0;
        let t12 = f64::sqrt(t10);
        let t14 = t11 + 1.86372 * t12 + 12.9352;
        let t15 = 1.0 / t14;
        let t19 = f64::ln(t4 * t9 * t15 / 4.0);
        let t20 = 0.0310907 * t19;
        let t21 = t12 + 3.72744;
        let t24 = f64::atan(6.15199081975908 / t21);
        let t25 = 0.038783294878113016 * t24;
        let t26 = t12 / 2.0;
        let t27 = t26 + 0.10498;
        let t28 = t27 * t27;
        let t30 = f64::ln(t28 * t15);
        let t31 = 0.0009690227711544374 * t30;
        let t33 = t11 + 3.53021 * t12 + 18.0578;
        let t34 = 1.0 / t33;
        let t38 = f64::ln(t4 * t9 * t34 / 4.0);
        let t40 = t12 + 7.06042;
        let t43 = f64::atan(4.730926909560113 / t40);
        let t45 = t26 + 0.325;
        let t46 = t45 * t45;
        let t48 = f64::ln(t46 * t34);
        let t50 = 0.01554535 * t38 + 0.05249139316978094 * t43 + 0.0022478670955426118 * t48 - t20 - t25 - t31;
        let t52 = t11 + 10.06155 * t12 + 101.578;
        let t53 = 1.0 / t52;
        let t57 = f64::ln(t4 * t9 * t53 / 4.0);
        let t59 = t12 + 20.1231;
        let t62 = f64::atan(1.171685277708993 / t59);
        let t64 = t26 + 0.743294;
        let t65 = t64 * t64;
        let t67 = f64::ln(t65 * t53);
        let t70 = t11 + 6.536 * t12 + 42.7198;
        let t71 = 1.0 / t70;
        let t75 = f64::ln(t4 * t9 * t71 / 4.0);
        let t77 = t12 + 13.072;
        let t80 = f64::atan(0.0448998886412873 / t77);
        let t82 = t26 + 0.409286;
        let t83 = t82 * t82;
        let t85 = f64::ln(t83 * t71);
        let t87 = 0.01554535 * t57 + 0.6188180297906063 * t62 + 0.002667310007273315 * t67 - 0.0310907 * t75 - 20.521972937837504 * t80 - 0.004431373767749538 * t85;
        let t88 = 1.0 / t87;
        let t90 = M_PI * M_PI;
        let t91 = 1.0 / t90;
        let t92 = t50 * t88 * t91;
        let t94 = t11 + 0.534175 * t12 + 11.4813;
        let t95 = 1.0 / t94;
        let t99 = f64::ln(t4 * t9 * t95 / 4.0);
        let t100 = t12 + 1.06835;
        let t103 = f64::atan(6.692072046645942 / t100);
        let t105 = t26 + 0.228344;
        let t106 = t105 * t105;
        let t108 = f64::ln(t106 * t95);
        let t110 = t99 + 0.32323836906055065 * t103 + 0.021608710360898266 * t108;
        let t112 = pow_1_3(zeta_threshold);
        let t114 = piecewise3(1.0 <= zeta_threshold, t112 * zeta_threshold, 1.0);
        let t116 = 2.0 * t114 - 2.0;
        let t118 = M_CBRT2;
        let t119 = t118 - 1.0;
        let t121 = 1.0 / t119 / 2.0;
        let t122 = 9.0 * t119;
        let t123 = t121 * t122;
        let t124 = t110 * t116 * t123;
        let t126 = t92 * t124 / 24.0;
        let tzk0 = t20 + t25 + t31 - t126;
        zk[ip] += tzk0;
    }
}
