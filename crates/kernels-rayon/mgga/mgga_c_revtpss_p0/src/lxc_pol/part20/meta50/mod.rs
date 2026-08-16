//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta50 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk340;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk341;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk342;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk343;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk344;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk345;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta50(t124: f64, t65: f64, t270: f64, t271: f64, t905: f64, t606: f64, t225: f64, t989: f64, t366: f64, t994: f64, t373: f64, t999: f64, t372: f64, t371: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1012 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk340(t124, t65);
        let t1014 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk341(t270, t271);
        let (t1015, t1016, t1017, t1020) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk342(t1014, t905, t606, t1012, t225, t989);
        let (t1021, t1024) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk343(t1020, t366, t225, t994);
        let t1025 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk344(t1024, t366);
        let (t1026, t1028) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk345(t373, t999, t372, t371);
    (t1012, t1014, t1015, t1016, t1017, t1020, t1021, t1024, t1025, t1026, t1028)
}
