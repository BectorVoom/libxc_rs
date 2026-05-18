//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1093/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1093<F: Float>(t12035: F, t6556: F, t41583: F, t41574: F, t41575: F, t41576: F, t41579: F, t41581: F, t41585: F, t46846: F, t46847: F, t46848: F, t47063: F) -> (F, F, F) {
    let t47064 = t6556 * t12035;
    let t47065 = F::new(2.0) * t47064;
    let t47066 = F::new(2.0) * t41583;
    let t47067 = -t46846 - t46847 + t46848 - t41574 - t41575 + t47063 - t41576 + t47065 - t41579 + t41581 + t47066 - t41585;
    (t47065, t47066, t47067)
}
