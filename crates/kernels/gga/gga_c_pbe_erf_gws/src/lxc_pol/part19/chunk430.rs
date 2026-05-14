//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 430/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk430<F: Float>(t1672: F, t220: F, t211: F, t633: F, t636: F, t218: F, t648: F) -> (F, F, F, F) {
    let t1778 = t1672 * t220;
    let t1780 = 4.0 / 135.0 * t211 * t1778;
    let t1788 = t633 * t636;
    let t1791 = 1.0 / t648 / t218;
    (t1778, t1780, t1788, t1791)
}
