//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta295 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1533;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1534;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1535;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1536;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1537;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta295(t11249: f64, t11631: f64, t1024: f64, t3105: f64, t3154: f64, t905: f64, t606: f64, t1052: f64, t360: f64, t3089: f64, t1087: f64, t3090: f64, t3278: f64, t3182: f64, t828: f64, t3109: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11632, t11656) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1533(t11249, t11631, t1024, t3105);
        let (t11660, t11661, t11670, t11671) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1534(t3154, t905, t606, t1052, t360, t3089);
        let t11672 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1535(t1087, t11671);
        let (t11675, t11703) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1536(t3090, t3278, t3182, t828);
        let t11710 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1537(t3109, t828);
    (t11632, t11656, t11660, t11661, t11670, t11671, t11672, t11675, t11703, t11710)
}
