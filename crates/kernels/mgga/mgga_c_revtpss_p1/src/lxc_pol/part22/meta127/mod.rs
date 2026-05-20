//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta127 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk845;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk846;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk847;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk848;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk849;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk850;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk851;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk852;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk853;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk854;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk855;
use chunk11::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk856;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta127<F: Float>(t3042: F, t341: F, t988: F, t993: F, t378: F, t989: F, t340: F, t992: F, t338: F, t999: F, t996: F, t1071: F, t994: F, t1096: F, t1079: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3043 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk845::<F>(t3042, t341);
        let t3046 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk846::<F>(t988, t993);
        let t3047 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk847::<F>(t3046, t378);
        let t3052 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk848::<F>(t378, t989);
        let t3056 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk849::<F>(t340, t992);
        let t3057 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk850::<F>(t3056, t338);
        let t3058 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk851::<F>(t3057, t378);
        let t3059 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk852::<F>(t999);
        let t3060 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk853::<F>(t3059, t996);
        let t3063 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk854::<F>(t1071, t994);
        let t3066 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk855::<F>(t1096, t999);
        let t3067 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk856::<F>(t1079, t3066);
    (t3043, t3046, t3047, t3052, t3056, t3057, t3058, t3059, t3060, t3063, t3066, t3067)
}
