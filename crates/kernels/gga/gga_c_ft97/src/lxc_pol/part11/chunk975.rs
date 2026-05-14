//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 975/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk975<F: Float>(t10896: F, t2253: F, t2953: F, t8640: F, t2941: F, t70: F, t9651: F, t327: F, t41536: F, t2934: F, t2920: F, t41762: F, t801: F, t272: F, t41670: F, t41622: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43188 = t2253 * t10896;
    let t43190 = t8640 * t2953;
    let t43192 = t8640 * t2941;
    let t43194 = t70 * t9651;
    let t43195 = t327 * t41536;
    let t43200 = t8640 * t2934;
    let t43202 = t8640 * t2920;
    let t43204 = t801 * t41762;
    let t43207 = 1.0 / t272 / t41670;
    let t43208 = t43207 * t41622;
    (t43188, t43190, t43192, t43194, t43195, t43200, t43202, t43204, t43208)
}
