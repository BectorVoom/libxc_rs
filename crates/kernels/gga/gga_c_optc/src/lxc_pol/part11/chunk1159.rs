//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1159/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1159<F: Float>(t17068: F, t2367: F, t930: F, t51450: F, t8194: F, t8197: F, t11380: F, t17141: F, t3917: F, t2679: F, t51745: F, t8114: F) -> (F, F, F, F) {
    let t51927 = t930 * t2367 * t17068;
    let t51930 = t8194 * t51450 * t8197;
    let t51935 = t3917 * t11380 * t17141;
    let t51938 = t8114 * t51745 * t2679;
    (t51927, t51930, t51935, t51938)
}
