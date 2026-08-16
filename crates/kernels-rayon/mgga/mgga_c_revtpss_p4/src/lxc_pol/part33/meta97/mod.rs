//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta97 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk622;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk623;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk624;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta97(t47: f64, t59: f64, t239: f64, t64: f64, t45: f64, t631: f64, t78: f64, t57: f64, t635: f64, t81: f64, t116: f64, t648: f64, t112: f64, t625: f64, t666: f64, t111: f64, t654: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2275, t2282) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk622(t47, t59);
        let (t2289, t2290, t2297, t2299, t2304, t2306, t2322) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk623(t239, t64, t45, t631, t78, t57, t635, t81, t116, t648);
        let (t2335, t2336, t2339) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk624(t112, t2289, t625, t666, t111, t654);
    (t2275, t2282, t2289, t2290, t2297, t2299, t2304, t2306, t2322, t2335, t2336, t2339)
}
