//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 756/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk756<F: Float>(t14538: F, t2721: F, t1382: F, t8384: F, t2434: F, t2367: F, t5021: F, t913: F, t14284: F, t953: F, t14279: F, t297: F, t4961: F) -> (F, F, F, F, F, F, F) {
    let t14539 = t2721 * t14538;
    let t14578 = t8384 * t1382;
    let t14585 = t2434 * t1382;
    let t14599 = t2367 * t5021;
    let t14600 = t913 * t14599;
    let t14617 = t953 * t14284;
    let t14619 = t953 * t14279;
    let t14630 = t4961 * t297;
    (t14539, t14578, t14585, t14600, t14617, t14619, t14630)
}
