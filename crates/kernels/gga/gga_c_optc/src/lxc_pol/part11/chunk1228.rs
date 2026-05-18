//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1228/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1228<F: Float>(t22074: F, t22079: F, t22098: F, t22103: F, t22107: F, t22111: F, t22115: F, t55882: F, t55883: F, t55884: F, t55944: F, t55977: F, t55980: F) -> F {
    let t56256 = t55882 - t55883 - t55884 - t55944 - t55977 + t22074 - t55980 + t22079 - t22098 - t22103 + t22107 + t22111 + t22115;
    t56256
}
