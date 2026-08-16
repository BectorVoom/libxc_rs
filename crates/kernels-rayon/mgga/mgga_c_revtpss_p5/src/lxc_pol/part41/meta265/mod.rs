//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta265 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1009;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta265(t138: f64, t9675: f64, t9674: f64, t4075: f64, t556: f64, t786: f64, t1444: f64, t2434: f64, t123: f64, t3915: f64, t1359: f64, t9292: f64, t1363: f64, t9288: f64, t1362: f64, t3911: f64, t3920: f64, t2237: f64, t240: f64, t550: f64, t816: f64, t1379: f64, t2689: f64, t3952: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9677, t9680, t9687, t9691) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1009(t138, t9675, t9674, t4075, t556, t786, t1444, t2434, t123, t3915, t1359, t9292);
        let (t9694, t9695, t9707, t9711, t9712) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1010(t1363, t9288, t1362, t3911, t3920, t2237, t240, t550, t816, t1379, t2689, t3952);
    (t9677, t9680, t9687, t9691, t9694, t9695, t9707, t9711, t9712)
}
