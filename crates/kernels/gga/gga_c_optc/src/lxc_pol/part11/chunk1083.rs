//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1083/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1083<F: Float>(t12869: F, t18054: F, t4464: F, t17697: F, t8487: F, t9169: F, t9171: F, t3109: F, t9175: F, t15911: F, t4488: F, t54753: F, t935: F, t1239: F, t1506: F, t3119: F, t5101: F) -> (F, F, F, F, F, F, F, F) {
    let t55027 = t4464 * t12869 * t18054;
    let t55037 = t8487 * t17697;
    let t55039 = t9169 * t55037 * t9171;
    let t55042 = t9175 * t55037 * t3109;
    let t55044 = t15911 * t4488;
    let t55067 = t54753 * t935;
    let t55127 = t1239 * t1506;
    let t55145 = t3119 * t5101;
    (t55027, t55037, t55039, t55042, t55044, t55067, t55127, t55145)
}
