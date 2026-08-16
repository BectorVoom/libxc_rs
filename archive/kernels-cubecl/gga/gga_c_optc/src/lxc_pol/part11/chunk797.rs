//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 797/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk797<F: Float>(t4990: F, t7947: F, t2641: F, t4941: F, t5025: F, t8152: F, t2721: F, t1382: F, t8384: F, t2434: F, t2367: F, t5021: F) -> (F, F, F, F, F, F) {
    let t14488 = t7947 * t4990;
    let t14525 = t2641 * t4941;
    let t14538 = t8152 * t5025;
    let t14539 = t2721 * t14538;
    let t14578 = t8384 * t1382;
    let t14585 = t2434 * t1382;
    let t14599 = t2367 * t5021;
    (t14488, t14525, t14539, t14578, t14585, t14599)
}
