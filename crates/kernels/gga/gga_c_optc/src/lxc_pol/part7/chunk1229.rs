//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1229/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1229<F: Float>(t22041: F, t3092: F, t3086: F, t8428: F, t22035: F, t3087: F, t1111: F, t24: F, t8538: F, t1113: F, t8414: F, t1122: F, t3103: F, t8471: F, t8487: F, t1900: F, t3119: F) -> (F, F, F, F, F, F, F) {
    let t27096 = t3092 * t22041;
    let t27100 = t3086 * t8428;
    let t27101 = t27100 * t22035;
    let t27105 = t3087 * t22041;
    let t27110 = t1111 * t24 * t8538;
    let t27112 = t1113 * t8414;
    let t27113 = t27112 * t22035;
    let t27119 = t3103 * t8487 * t1122 * t8471;
    let t27122 = t1900 * t1122 * t3119;
    (t27096, t27101, t27105, t27110, t27113, t27119, t27122)
}
