//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3144/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3144(t1794: f64, t5819: f64, t17459: f64, t23842: f64, t5405: f64, t24610: f64, t21242: f64, t5378: f64, t1785: f64, t21271: f64, t1261: f64, t24248: f64, t247: f64, t3634: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t82578 = t5819 * t1794;
    let t82579 = t82578 * t17459;
    let t82587 = t23842 * t5405;
    let t82591 = t24610 * t5405;
    let t82595 = t21242 * t5378;
    let t82597 = t1785 * t21271;
    let t82603 = t1261 * t247 * t3634 * t24248;
    (t82578, t82579, t82587, t82591, t82595, t82597, t82603)
}
