//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta485 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1768;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1769;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1770;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1771;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1772;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1773;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta485(t122: f64, t1949: f64, t72: f64, t2466: f64, t25375: f64, t1955: f64, t25308: f64, t251: f64, t7063: f64, t25374: f64, t2769: f64, t7056: f64, t822: f64, t1950: f64, t867: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25377, t25378, t25379, t25383) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1768(t122, t1949, t72, t2466, t25375, t1955, t25308);
        let t25386 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1769(t251, t7063);
        let t25387 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1770(t25374, t25386);
        let (t25388, t25390, t25391) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1771(t25378, t25387, t2769, t7056, t1955);
        let t25392 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1772(t1949, t822);
        let (t25398, t25399) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1773(t1950, t867, t786);
    (t25377, t25378, t25379, t25383, t25386, t25387, t25388, t25390, t25391, t25392, t25398, t25399)
}
