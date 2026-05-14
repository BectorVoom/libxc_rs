//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1041/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1041<F: Float>(t1940: F, t9493: F, t1915: F, t3559: F, t5728: F, t759: F, t2099: F, t757: F, t9541: F, t17848: F, t2104: F, t9288: F, t5974: F, t9558: F, t2899: F, t774: F, t9563: F) -> (F, F, F, F, F, F, F) {
    let t26336 = t9493 * t1940;
    let t26357 = t3559 * t1915;
    let t26387 = t5728 * t759;
    let t26413 = t757 * t2099 * t9541;
    let t26423 = t2104 * t17848 * t9288;
    let t26426 = t2104 * t5974 * t9558;
    let t26430 = t2899 * t774 * t9563;
    (t26336, t26357, t26387, t26413, t26423, t26426, t26430)
}
