//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1209/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1209<F: Float>(t443: F, t6225: F, t1385: F, t332: F, t439: F, t4663: F, t6494: F, t1447: F, t6387: F, t6391: F, t517: F, t6130: F) -> (F, F, F, F, F) {
    let t15935 = t6225 * t443;
    let t15939 = F::new(2.0) / F::new(45.0) * t439 * t1385 * t15935 * t332;
    let t15942 = F::new(4.0) / F::new(15.0) * t439 * t6494 * t4663;
    let t15943 = t1447 * t6387;
    let t15944 = F::new(8.0) / F::new(135.0) * t15943;
    let t15945 = t1447 * t6391;
    let t15946 = F::new(8.0) / F::new(135.0) * t15945;
    let t15947 = t6130 * t517;
    (t15939, t15942, t15944, t15946, t15947)
}
