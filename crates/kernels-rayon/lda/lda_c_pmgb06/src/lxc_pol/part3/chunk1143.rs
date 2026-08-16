//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1143/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1143(t405: f64, t5016: f64, t5019: f64, t4913: f64, t5022: f64, t5010: f64, t5013: f64, t103: f64, t12146: f64, t12150: f64, t12176: f64, t12181: f64, t12339: f64, t12343: f64, t12389: f64, t12391: f64, t12393: f64, t12400: f64, t12404: f64, t12408: f64, t1619: f64, t2060: f64, t3404: f64, t473: f64, t9724: f64, t9737: f64, t9739: f64) -> f64 {
    let t13633 = t405 * t5016;
    let t13635 = t405 * t5019;
    let t13637 = t4913 * t5022;
    let t13639 = t405 * t5010;
    let t13644 = t405 * t5013;
    let t13662 = 0.035555555555555556_f64 * t103 * t3404 * t12176 + 0.08_f64 * t2060 * t1619 * t12181 - 0.24_f64 * t2060 * t473 * t12343 + 0.0044444444444444444_f64 * t13633 + 0.005925925925925926_f64 * t13635 + 0.057777777777777775_f64 * t13637 - 0.02666666666666667_f64 * t13639 - 0.08_f64 * t103 * t1619 * t12339 + 0.08_f64 * t13644 + 0.16_f64 * t103 * t473 * t12389 + 0.8638_f64 * t12391 - 0.14396666666666666_f64 * t12393 + t9724 - 0.0022222222222222222_f64 * t103 * t1619 * t12146 - 0.013333333333333334_f64 * t2060 * t1619 * t12150 - 0.02666666666666667_f64 * t9737 + 0.0044444444444444444_f64 * t9739 - 0.11997222222222222_f64 * t12400 - 0.64785_f64 * t12404 + 0.4319_f64 * t12408;
    t13662
}
