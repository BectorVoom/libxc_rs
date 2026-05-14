//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 764/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk764<F: Float>(t326: F, t6469: F, t2200: F, t855: F, t859: F, t854: F, t2087: F, t2142: F, t899: F, t912: F, t923: F, t2348: F, t2251: F, t916: F, t2250: F, t814: F, t875: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6608 = t326 * t6469;
    let t6616 = t855 * t2200 * t859;
    let t6617 = t854 * t6616;
    let t6624 = t2087 * t2142;
    let t6627 = t899 * t912 * t923;
    let t6628 = t6627 * t2348;
    let t6636 = t2251 * t916;
    let t6637 = t2250 * t6636;
    let t6638 = t875 * t814;
    (t6608, t6616, t6617, t6624, t6627, t6628, t6636, t6637, t6638)
}
