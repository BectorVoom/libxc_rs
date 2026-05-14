//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 803/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk803<F: Float>(t24964: F, t824: F, t193: F, t89: F, t2739: F, t6222: F, t6308: F, t6310: F, t681: F, t2781: F, t683: F) -> (F, F, F, F, F, F) {
    let t24965 = t24964 * t824;
    let t24966 = t193 * t24965;
    let t24967 = t89 * t24966;
    let t24969 = t6222 * t2739;
    let t24970 = t193 * t24969;
    let t24971 = t89 * t24970;
    let t24974 = t6308 * t681 * t6310;
    let t24976 = t683 * t2781;
    (t24965, t24967, t24969, t24971, t24974, t24976)
}
