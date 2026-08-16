//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 989/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk989<F: Float>(t22242: F, t8392: F, t1882: F, t22357: F, t22369: F, t22205: F, t22251: F, t22439: F, t870: F, t22454: F, t22346: F, t22178: F) -> (F, F, F, F, F, F, F, F, F) {
    let t84390 = t8392 * t22242;
    let t84404 = t1882 * t22357;
    let t84486 = t8392 * t22369;
    let t84500 = t1882 * t22205;
    let t84504 = t1882 * t22251;
    let t84519 = t22439 * t870;
    let t84547 = t1882 * t22454;
    let t84581 = t870 * t22346;
    let t84586 = t8392 * t22178;
    (t84390, t84404, t84486, t84500, t84504, t84519, t84547, t84581, t84586)
}
