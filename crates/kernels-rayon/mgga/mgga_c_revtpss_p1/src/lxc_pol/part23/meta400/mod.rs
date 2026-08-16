//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta400 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1766;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1767;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1768;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1769;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta400(t17727: f64, t17728: f64, t3566: f64, t489: f64, t1121: f64, t1774: f64, t13142: f64, t17708: f64, t13127: f64) -> (f64, f64, f64, f64, f64, f64) {
        let t17729 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1766(t17727, t17728);
        let (t17735, t17736) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1767(t3566, t489, t17728);
        let (t17737, t17747) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1768(t1121, t1774, t13142, t17708);
        let t17753 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1769(t13127, t17708);
    (t17729, t17735, t17736, t17737, t17747, t17753)
}
