//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1545;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1546;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1547;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1548;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta298(t3154: f64, t999: f64, t1086: f64, t3046: f64, t3090: f64, t3316: f64, t994: f64, t4891: f64, t1016: f64, t697: f64, t1011: f64, t11132: f64, t126: f64, t373: f64, t828: f64, t3057: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11860, t11865, t11866, t11874, t11875) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1545(t3154, t999, t1086, t3046, t3090, t3316, t994, t4891);
        let (t11881, t11890, t11921) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1546(t1016, t697, t1011, t11132, t126, t373);
        let t11922 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1547(t11921, t828);
        let (t11926, t11927) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1548(t1086, t3057, t3090);
    (t11860, t11865, t11866, t11874, t11875, t11881, t11890, t11921, t11922, t11926, t11927)
}
