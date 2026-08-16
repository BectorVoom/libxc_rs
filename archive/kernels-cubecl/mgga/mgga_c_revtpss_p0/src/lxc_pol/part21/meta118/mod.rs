//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta118 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk766;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk767;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk768;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk769;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk770;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk771;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk772;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk773;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk774;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta118<F: Float>(t2846: F, t689: F, t907: F, t1065: F, t159: F, t631: F, t2251: F, t128: F, t2297: F, t904: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2847, t2848) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk766::<F>(t2846, t689, t907);
        let t2850 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk767::<F>(t1065, t159);
        let t2851 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk768::<F>(t631);
        let t2852 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk769::<F>(t2851);
        let t2853 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk770::<F>(t2251, t2852);
        let (t2854, t2855) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk771::<F>(t2850, t2853, t128);
        let t2857 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk772::<F>(t2297);
        let t2858 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk773::<F>(t2251, t2857);
        let (t2859, t2860) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk774::<F>(t2858, t904, t128);
    (t2847, t2848, t2850, t2851, t2852, t2853, t2854, t2855, t2857, t2858, t2859, t2860)
}
