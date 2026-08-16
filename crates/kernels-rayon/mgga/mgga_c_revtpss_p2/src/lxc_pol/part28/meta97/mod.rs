//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta97 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk619;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk620;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk621;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk622;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk623;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta97(t22: f64, t2224: f64, t584: f64, t588: f64, t20: f64, t27: f64, t12: f64, t19: f64, t592: f64, t596: f64, t21: f64, t25: f64, t2219: f64, t2221: f64, t2223: f64, t599: f64, t602: f64, t89: f64, t90: f64, t29: f64, t644: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2226, t2228, t2230, t2231, t2233, t2235, t2236, t2237, t2239) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk619(t22, t2224, t584, t588, t20, t27, t12, t19, t592, t596, t21, t25);
        let (t2240, t2242) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk620(t2219, t2221, t2223, t2226, t2228, t2230, t2233, t2235, t2239, t599, t602);
        let t2246 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk621(t89, t90);
        let t2247 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk622(t2246, t29);
        let t2248 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk623(t644);
    (t2226, t2230, t2231, t2233, t2236, t2237, t2239, t2240, t2242, t2246, t2247, t2248)
}
