//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1094/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1094<F: Float>(t2018: F, t2563: F, t16558: F, t439: F, t5482: F, t6412: F, t6160: F, t6494: F, t6165: F, t6498: F, t13933: F, t6464: F) -> (F, F, F, F, F, F) {
    let t20160 = t2563 * t2018;
    let t20161 = t20160 / F::cast_from(15.0_f64);
    let t20162 = t16558 / F::cast_from(15.0_f64);
    let t20165 = t439 * t5482 * t6412 / F::cast_from(15.0_f64);
    let t20168 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t439 * t6494 * t6160;
    let t20171 = t439 * t6498 * t6165 / F::cast_from(9.0_f64);
    let t20174 = t439 * t13933 * t6464 / F::cast_from(9.0_f64);
    (t20161, t20162, t20165, t20168, t20171, t20174)
}
