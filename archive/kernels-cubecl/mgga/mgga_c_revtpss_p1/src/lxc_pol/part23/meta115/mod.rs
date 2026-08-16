//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta115 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk746;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk747;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk748;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk749;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk750;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk751;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk752;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk753;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta115<F: Float>(t300: F, t960: F, t2846: F, t988: F, t993: F, t378: F, t989: F, t340: F, t992: F, t338: F, t1071: F, t994: F) -> (F, F, F, F, F, F, F, F, F) {
        let t3022 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk746::<F>(t300, t960);
        let (t3037, t3046) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk747::<F>(t2846, t988, t993);
        let t3047 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk748::<F>(t3046, t378);
        let t3052 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk749::<F>(t378, t989);
        let t3056 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk750::<F>(t340, t992);
        let t3057 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk751::<F>(t3056, t338);
        let t3058 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk752::<F>(t3057, t378);
        let t3063 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk753::<F>(t1071, t994);
    (t3022, t3037, t3046, t3047, t3052, t3056, t3057, t3058, t3063)
}
