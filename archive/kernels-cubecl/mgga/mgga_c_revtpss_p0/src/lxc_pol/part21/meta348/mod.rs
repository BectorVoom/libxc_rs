//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1683;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1684;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1685;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1686;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta348<F: Float>(t3154: F, t905: F, t606: F, t11659: F, t3092: F, t3095: F, t1052: F, t360: F, t3089: F, t1087: F, t3090: F, t3278: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11661, t11662, t11663, t11666, t11667, t11670) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1683::<F>(t3154, t905, t606, t11659, t3092, t3095, t1052, t360);
        let t11671 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1684::<F>(t11670, t3089);
        let t11672 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1685::<F>(t1087, t11671);
        let t11675 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1686::<F>(t3090, t3278);
    (t11661, t11662, t11663, t11666, t11667, t11670, t11671, t11672, t11675)
}
