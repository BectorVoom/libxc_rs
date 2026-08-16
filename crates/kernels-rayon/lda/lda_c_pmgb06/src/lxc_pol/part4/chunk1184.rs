//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1184/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1184(t405: f64, t6193: f64, t6147: f64, t4913: f64, t6196: f64, t6199: f64, t6202: f64, t103: f64, t15353: f64, t15360: f64, t15365: f64, t15369: f64, t15395: f64, t15440: f64, t15445: f64, t1619: f64, t473: f64, t9715: f64) -> f64 {
    let t15589 = t405 * t6193;
    let t15591 = t405 * t6147;
    let t15593 = t4913 * t6196;
    let t15601 = t405 * t6199;
    let t15603 = t405 * t6202;
    let t15611 = 0.14396666666666666_f64 * t15360 - 0.047988888888888886_f64 * t15365 - 0.023994444444444443_f64 * t15369 + 0.05333333333333334_f64 * t15589 - 0.017777777777777778_f64 * t15591 - 0.2311111111111111_f64 * t15593 - 0.04_f64 * t103 * t473 * t15395 - 0.08_f64 * t103 * t1619 * t15440 - 0.017777777777777778_f64 * t15601 + 0.002962962962962963_f64 * t15603 + 0.02666666666666667_f64 * t103 * t473 * t15445 + 0.013333333333333334_f64 * t103 * t473 * t15353 + t9715;
    t15611
}
