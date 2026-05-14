//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1011/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1011<F: Float>(t13709: F, t13714: F, t13720: F, t20863: F, t20866: F, t20870: F, t20872: F, t20874: F, t20877: F, t20879: F, t20881: F, t20884: F, t2497: F, t5305: F, t1972: F, t6387: F) -> (F, F, F) {
    let t20885 = t20863 + t20866 + t20870 - t13709 - t13714 - t13720 + t20872 + t20874 + t20877 + t20879 - t20881 - t20884;
    let t20888 = 2.0 / 15.0 * t5305 * t2497;
    let t20890 = 2.0 / 15.0 * t1972 * t6387;
    (t20885, t20888, t20890)
}
