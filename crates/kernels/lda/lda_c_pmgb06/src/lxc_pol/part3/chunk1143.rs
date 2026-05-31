//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1143/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1143<F: Float>(t405: F, t5016: F, t5019: F, t4913: F, t5022: F, t5010: F, t5013: F, t103: F, t12146: F, t12150: F, t12176: F, t12181: F, t12339: F, t12343: F, t12389: F, t12391: F, t12393: F, t12400: F, t12404: F, t12408: F, t1619: F, t2060: F, t3404: F, t473: F, t9724: F, t9737: F, t9739: F) -> F {
    let t13633 = t405 * t5016;
    let t13635 = t405 * t5019;
    let t13637 = t4913 * t5022;
    let t13639 = t405 * t5010;
    let t13644 = t405 * t5013;
    let t13662 = F::cast_from(0.035555555555555556_f64) * t103 * t3404 * t12176 + F::cast_from(0.08_f64) * t2060 * t1619 * t12181 - F::cast_from(0.24_f64) * t2060 * t473 * t12343 + F::cast_from(0.0044444444444444444_f64) * t13633 + F::cast_from(0.005925925925925926_f64) * t13635 + F::cast_from(0.057777777777777775_f64) * t13637 - F::cast_from(0.02666666666666667_f64) * t13639 - F::cast_from(0.08_f64) * t103 * t1619 * t12339 + F::cast_from(0.08_f64) * t13644 + F::cast_from(0.16_f64) * t103 * t473 * t12389 + F::cast_from(0.8638_f64) * t12391 - F::cast_from(0.14396666666666666_f64) * t12393 + t9724 - F::cast_from(0.0022222222222222222_f64) * t103 * t1619 * t12146 - F::cast_from(0.013333333333333334_f64) * t2060 * t1619 * t12150 - F::cast_from(0.02666666666666667_f64) * t9737 + F::cast_from(0.0044444444444444444_f64) * t9739 - F::cast_from(0.11997222222222222_f64) * t12400 - F::cast_from(0.64785_f64) * t12404 + F::cast_from(0.4319_f64) * t12408;
    t13662
}
