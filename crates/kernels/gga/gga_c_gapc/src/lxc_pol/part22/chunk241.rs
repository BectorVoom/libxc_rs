//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 241/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk241<F: Float>(t191: F, t933: F, t332: F, t786: F, t330: F, t197: F, t325: F, t641: F, t6: F) -> (F, F, F, F, F, F) {
    let t934 = t933 * t191;
    let t935 = t332 * t786;
    let t936 = t330 * t935;
    let t937 = t197 * t936;
    let t940 = t325 * t641;
    let t941 = t332 * t6;
    (t934, t935, t936, t937, t940, t941)
}
