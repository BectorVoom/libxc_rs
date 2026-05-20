//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta126 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk807;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk808;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk809;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk810;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk811;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk812;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk813;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk814;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta126<F: Float>(t2846: F, t2904: F, t2848: F, t2855: F, t2860: F, t2864: F, t2882: F, t2890: F, t2898: F, t2900: F, t2906: F, t2910: F, t2913: F, t2916: F, t973: F, t963: F, t315: F, t323: F, t2988: F, t2868: F, t2871: F, t2878: F, t2921: F, t2929: F, t2935: F, t2938: F, t2943: F, t2945: F, t2963: F, t2968: F, t2971: F, t2980: F, t2982: F, t2987: F, t2989: F, t311: F, t946: F, t955: F, t965: F, t974: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2994, t3001, t3006) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk807::<F>(t2846, t2904, t2848, t2855, t2860, t2864, t2882, t2890, t2898, t2900, t2906, t2910, t2913, t2916);
        let t3007 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk808::<F>(t3006, t973);
        let t3010 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk809::<F>(t963);
        let t3011 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk810::<F>(t3010);
        let t3012 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk811::<F>(t3011, t315);
        let (t3013, t3014) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk812::<F>(t323);
        let t3015 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk813::<F>(t2988, t3014);
        let t3018 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk814::<F>(t2868, t2871, t2878, t2921, t2929, t2935, t2938, t2943, t2945, t2963, t2968, t2971, t2980, t2982, t2987, t2989, t3007, t3012, t3015, t311, t946, t955, t965, t974);
    (t2994, t3001, t3006, t3007, t3010, t3011, t3012, t3013, t3014, t3015, t3018)
}
