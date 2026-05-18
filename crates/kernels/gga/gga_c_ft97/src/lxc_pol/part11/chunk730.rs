//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 730/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk730<F: Float>(t737: F, t9909: F, t3917: F, t9592: F, t2: F, t9802: F, t9745: F, t9577: F, t9571: F, t2486: F, t3910: F, t9583: F) -> (F, F, F, F, F, F, F, F) {
    let t9910 = t737 * t9909;
    let t9913 = t3917 * t9592;
    let t9916 = t9802 * t2;
    let t9917 = t9916 * t9745;
    let t9920 = t2 * t9577;
    let t9921 = t9920 * t9571;
    let t9922 = t2486 * t9921;
    let t9925 = t3910 * t9583;
    (t9910, t9913, t9916, t9917, t9920, t9921, t9922, t9925)
}
