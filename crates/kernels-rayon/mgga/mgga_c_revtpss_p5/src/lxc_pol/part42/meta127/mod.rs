//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta127 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk620;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk621;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk622;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk623;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk624;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta127(t3316: f64, t342: f64, t3303: f64, t357: f64, t389: f64, t1941: f64, t268: f64, t404: f64, t1123: f64, t689: f64, t1263: f64, t159: f64, t635: f64, t2304: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3317, t3318, t3335, t3336, t3356) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk620(t3316, t342, t3303, t357, t389, t1941, t268, t404);
        let (t3357, t3358) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk621(t3356, t1123, t689);
        let t3360 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk622(t1263, t159);
        let (t3361, t3362) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk623(t635);
        let t3367 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk624(t2304);
    (t3317, t3318, t3335, t3336, t3356, t3357, t3358, t3360, t3361, t3362, t3367)
}
