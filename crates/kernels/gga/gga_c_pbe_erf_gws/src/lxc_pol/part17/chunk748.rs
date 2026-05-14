//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 748/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk748<F: Float>(t2100: F, t817: F, t2106: F, t814: F, t816: F, t322: F, t1452: F, t823: F, t825: F, t897: F) -> (F, F, F, F, F, F, F) {
    let t6086 = t2100 * t817;
    let t6089 = t814 * t2106;
    let t6094 = t816 * t816;
    let t6095 = 1.0 / t6094;
    let t6096 = t322 * t6095;
    let t6110 = t823 * t1452;
    let t6111 = t6110 * t825;
    let t6125 = t897 * t897;
    let t6126 = 1.0 / t6125;
    (t6086, t6089, t6096, t6110, t6111, t6125, t6126)
}
