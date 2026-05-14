//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1188/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1188<F: Float>(t2587: F, t5264: F, t17053: F, t2655: F, t1730: F, t20199: F, t2648: F, t6870: F, t6892: F, t1769: F, t7001: F, t16324: F, t177: F, t6992: F, t2642: F, t5384: F) -> (F, F, F, F, F, F, F, F) {
    let t20221 = t5264 * t2587;
    let t20242 = t17053 * t2655;
    let t20261 = t1730 * t20199 * t2648;
    let t20263 = t6892 * t6870;
    let t20265 = t1769 * t7001;
    let t20267 = t16324 * t177;
    let t20272 = t1769 * t6992;
    let t20274 = t5384 * t2642;
    (t20221, t20242, t20261, t20263, t20265, t20267, t20272, t20274)
}
