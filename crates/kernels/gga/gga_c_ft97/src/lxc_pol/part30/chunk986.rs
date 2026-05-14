//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 986/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk986<F: Float>(t127680: F, t35917: F, t35504: F, t52: F, t811: F, t150590: F, t31465: F, t140943: F, t33941: F, t35932: F, t33934: F, t35928: F, t150658: F, t7009: F, t153074: F, t800: F) -> (F, F, F, F, F, F, F) {
    let t153188 = t35917 * t127680;
    let t153193 = t52 * t35504 * t811;
    let t153196 = t31465 * t150590;
    let t153205 = t33941 * t140943 * t35932;
    let t153208 = t33934 * t140943 * t35928;
    let t153210 = t7009 * t150658;
    let t153216 = t800 * t153074;
    (t153188, t153193, t153196, t153205, t153208, t153210, t153216)
}
