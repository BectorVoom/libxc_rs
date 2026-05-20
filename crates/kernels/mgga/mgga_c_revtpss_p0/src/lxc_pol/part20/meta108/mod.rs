//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta108 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk613;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk614;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk615;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk616;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk617;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk618;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk619;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk620;
use chunk8::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk621;
use chunk9::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk622;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta108<F: Float>(t3042: F, t341: F, t988: F, t993: F, t378: F, t989: F, t340: F, t992: F, t338: F, t999: F, t996: F, t1071: F, t994: F, t1096: F, t1079: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3043 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk613::<F>(t3042, t341);
        let t3046 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk614::<F>(t988, t993);
        let t3047 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk615::<F>(t3046, t378);
        let t3052 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk616::<F>(t378, t989);
        let (t3056, t3057) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk617::<F>(t340, t992, t338);
        let t3058 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk618::<F>(t3057, t378);
        let t3059 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk619::<F>(t999);
        let t3060 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk620::<F>(t3059, t996);
        let t3063 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk621::<F>(t1071, t994);
        let (t3066, t3067) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk622::<F>(t1096, t999, t1079);
    (t3043, t3046, t3047, t3052, t3056, t3057, t3058, t3059, t3060, t3063, t3066, t3067)
}
