//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1297/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1297<F: Float>(t2620: F, t955: F, t1414: F, t79: F, t405: F, t6879: F, t1464: F, t14073: F, t14078: F, t14080: F, t14082: F, t15650: F, t1576: F, t17025: F, t17030: F, t1820: F, t1825: F, t2060: F, t525: F) -> (F, F, F) {
    let t17035 = t955 * t2620;
    let t17037 = t1414 * t79;
    let t17041 = t405 * t6879;
    let t17043 = t1464 * t79;
    let t17047 = F::new(0.05333333333333334) * t14073 - F::new(0.017777777777777778) * t14078 + F::new(0.003950617283950617) * t14080 - F::new(0.011851851851851851) * t14082 - F::new(0.0024691358024691358) * t17025 + F::new(0.008888888888888889) * t2060 * t1576 * t1820 + F::new(0.014814814814814815) * t17030 - F::new(0.05333333333333334) * t2060 * t525 * t1825 - F::new(0.007407407407407408) * t17035 + F::new(0.10666666666666667) * t15650 * t525 * t17037 + F::new(0.008888888888888889) * t17041 - F::new(0.017777777777777778) * t15650 * t1576 * t17043;
    (t17037, t17043, t17047)
}
