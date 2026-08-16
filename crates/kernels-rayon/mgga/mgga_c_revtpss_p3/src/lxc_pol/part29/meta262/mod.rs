//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta262 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1076;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1077;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1078;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1079;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1080;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta262(t1955: f64, t860: f64, t7056: f64, t233: f64, t2769: f64, t822: f64, t867: f64, t30: f64, t890: f64, t33: f64, t775: f64, t1315: f64, t196: f64, t197: f64, t1353: f64, t1450: f64, t533: f64, t7021: f64, t816: f64, t1941: f64, t540: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7067, t7070) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1076(t1955, t860, t7056);
        let t7071 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1077(t233, t2769);
        let t7076 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1078(t822, t867);
        let (t7092, t7200, t7207, t7234, t7235) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1079(t30, t890, t33, t775, t1315, t196, t197);
        let (t7238, t7250, t7252) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1080(t1353, t1450, t533, t7021, t816, t1941, t540);
    (t7067, t7070, t7071, t7076, t7092, t7200, t7207, t7234, t7235, t7238, t7250, t7252)
}
