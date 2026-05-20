//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta128 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk817;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk818;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk819;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk820;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk821;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk822;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk823;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk824;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk825;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk826;
use chunk10::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk827;
use chunk11::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk828;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta128<F: Float>(t3042: F, t341: F, t988: F, t993: F, t378: F, t989: F, t340: F, t992: F, t338: F, t999: F, t996: F, t1071: F, t994: F, t1096: F, t1079: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3043 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk817::<F>(t3042, t341);
        let t3046 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk818::<F>(t988, t993);
        let t3047 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk819::<F>(t3046, t378);
        let t3052 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk820::<F>(t378, t989);
        let t3056 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk821::<F>(t340, t992);
        let t3057 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk822::<F>(t3056, t338);
        let t3058 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk823::<F>(t3057, t378);
        let t3059 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk824::<F>(t999);
        let t3060 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk825::<F>(t3059, t996);
        let t3063 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk826::<F>(t1071, t994);
        let t3066 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk827::<F>(t1096, t999);
        let t3067 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk828::<F>(t1079, t3066);
    (t3043, t3046, t3047, t3052, t3056, t3057, t3058, t3059, t3060, t3063, t3066, t3067)
}
