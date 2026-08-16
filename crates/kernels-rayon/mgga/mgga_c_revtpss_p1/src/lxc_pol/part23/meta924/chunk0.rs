//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2988/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2988(t1063: f64, t23485: f64, t247: f64, t3109: f64, t11922: f64, t23993: f64, t3115: f64, t3181: f64, t372: f64, t6305: f64, t23935: f64, t4899: f64) -> (f64, f64, f64, f64) {
    let t79219 = t1063 * t247 * t3109 * t23485;
    let t79233 = t3115 * t11922 * t23993;
    let t79247 = t372 * t3181 * t6305;
    let t79253 = t4899 * t11922 * t23935;
    (t79219, t79233, t79247, t79253)
}
