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
    let t13662 = F::new(0.035555555555555556) * t103 * t3404 * t12176 + F::new(0.08) * t2060 * t1619 * t12181 - F::new(0.24) * t2060 * t473 * t12343 + F::new(0.0044444444444444444) * t13633 + F::new(0.005925925925925926) * t13635 + F::new(0.057777777777777775) * t13637 - F::new(0.02666666666666667) * t13639 - F::new(0.08) * t103 * t1619 * t12339 + F::new(0.08) * t13644 + F::new(0.16) * t103 * t473 * t12389 + F::new(0.8638) * t12391 - F::new(0.14396666666666666) * t12393 + t9724 - F::new(0.0022222222222222222) * t103 * t1619 * t12146 - F::new(0.013333333333333334) * t2060 * t1619 * t12150 - F::new(0.02666666666666667) * t9737 + F::new(0.0044444444444444444) * t9739 - F::new(0.11997222222222222) * t12400 - F::new(0.64785) * t12404 + F::new(0.4319) * t12408;
    t13662
}
