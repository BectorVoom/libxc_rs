//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta117 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk791;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk792;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk793;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk794;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk795;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk796;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk797;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk798;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk799;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk800;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk801;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta117<F: Float>(t2498: F, t2518: F, t2522: F, t2525: F, t2527: F, t2562: F, t2579: F, t2587: F, t2610: F, t2621: F, t2624: F, t2628: F, t2632: F, t2836: F, t1941: F, t268: F, t271: F, t689: F, t907: F, t1065: F, t159: F, t631: F, t2251: F, t128: F, t2297: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t2837 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk791::<F>(t2498, t2518, t2522, t2525, t2527, t2562, t2579, t2587, t2610, t2621, t2624, t2628, t2632);
        let t2838 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk792::<F>(t2836, t2837);
        let t2846 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk793::<F>(t1941, t268, t271);
        let (t2847, t2848) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk794::<F>(t2846, t689, t907);
        let t2850 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk795::<F>(t1065, t159);
        let t2851 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk796::<F>(t631);
        let t2852 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk797::<F>(t2851);
        let t2853 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk798::<F>(t2251, t2852);
        let (t2854, t2855) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk799::<F>(t2850, t2853, t128);
        let t2857 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk800::<F>(t2297);
        let t2858 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk801::<F>(t2251, t2857);
    (t2838, t2846, t2847, t2848, t2850, t2851, t2852, t2853, t2854, t2855, t2857, t2858)
}
