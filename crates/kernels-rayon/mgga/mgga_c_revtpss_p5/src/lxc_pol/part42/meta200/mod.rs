//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta200 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk807;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk808;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk809;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk810;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta200(t1715: f64, t3634: f64, t247: f64, t1261: f64, t1260: f64, t1785: f64, t3670: f64, t3719: f64, t5230: f64, t1802: f64, t369: f64, t475: f64, t467: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t5378, t5379, t5381) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk807(t1715, t3634, t247, t1261, t1260, t1785);
        let t5384 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk808(t1260, t3670);
        let (t5386, t5390) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk809(t3719, t5230, t247, t1802, t369, t475);
        let t5391 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk810(t467, t5390);
    (t5378, t5379, t5381, t5384, t5386, t5390, t5391)
}
