//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1297/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1297(t2620: f64, t955: f64, t1414: f64, t79: f64, t405: f64, t6879: f64, t1464: f64, t14073: f64, t14078: f64, t14080: f64, t14082: f64, t15650: f64, t1576: f64, t17025: f64, t17030: f64, t1820: f64, t1825: f64, t2060: f64, t525: f64) -> (f64, f64, f64) {
    let t17035 = t955 * t2620;
    let t17037 = t1414 * t79;
    let t17041 = t405 * t6879;
    let t17043 = t1464 * t79;
    let t17047 = 0.05333333333333334_f64 * t14073 - 0.017777777777777778_f64 * t14078 + 0.003950617283950617_f64 * t14080 - 0.011851851851851851_f64 * t14082 - 0.0024691358024691358_f64 * t17025 + 0.008888888888888889_f64 * t2060 * t1576 * t1820 + 0.014814814814814815_f64 * t17030 - 0.05333333333333334_f64 * t2060 * t525 * t1825 - 0.007407407407407408_f64 * t17035 + 0.10666666666666667_f64 * t15650 * t525 * t17037 + 0.008888888888888889_f64 * t17041 - 0.017777777777777778_f64 * t15650 * t1576 * t17043;
    (t17037, t17043, t17047)
}
