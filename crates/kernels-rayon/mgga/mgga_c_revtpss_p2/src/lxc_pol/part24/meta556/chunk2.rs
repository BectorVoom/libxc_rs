//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1663/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1663(t88188: f64, t88201: f64, t916: f64, t6113: f64, t41401: f64, t141: f64, t2908: f64, t88132: f64, t41382: f64, t6120: f64, t2897: f64, t2880: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t88202 = t88188 + t88201;
    let t88203 = t916 * t88202;
    let t88205 = t6113 * t6113;
    let t88206 = t41401 * t88205;
    let t88209 = t141 * t2908 * t88132;
    let t88211 = t41382 * t88205;
    let t88213 = t6120 * t6120;
    let t88214 = t2897 * t88213;
    let t88216 = t2880 * t88213;
    (t88202, t88203, t88206, t88209, t88211, t88214, t88216)
}
