//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta104 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk593;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk594;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk595;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk596;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk597;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk598;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta104(t3520: f64, t439: f64, t447: f64, t3356: f64, t1207: f64, t458: f64, t456: f64, t487: f64, t3140: f64, t460: f64, t1242: f64, t472: f64, t474: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3521 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk593(t3520, t439);
        let (t3522, t3523) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk594(t447);
        let (t3546, t3565, t3566) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk595(t3356, t1207, t458, t456);
        let t3567 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk596(t3566, t487);
        let (t3579, t3594) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk597(t3356, t3140, t460);
        let (t3596, t3597) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk598(t1242, t472, t474);
    (t3521, t3522, t3523, t3546, t3565, t3566, t3567, t3579, t3594, t3596, t3597)
}
