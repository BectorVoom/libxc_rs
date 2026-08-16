//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1683;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1684;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1685;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1686;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta348(t3154: f64, t905: f64, t606: f64, t11659: f64, t3092: f64, t3095: f64, t1052: f64, t360: f64, t3089: f64, t1087: f64, t3090: f64, t3278: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11661, t11662, t11663, t11666, t11667, t11670) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1683(t3154, t905, t606, t11659, t3092, t3095, t1052, t360);
        let t11671 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1684(t11670, t3089);
        let t11672 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1685(t1087, t11671);
        let t11675 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1686(t3090, t3278);
    (t11661, t11662, t11663, t11666, t11667, t11670, t11671, t11672, t11675)
}
