//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1279/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1279(t4144: f64, t9593: f64, t159: f64, t2698: f64, t4135: f64, t4147: f64, t26: f64, t65: f64, t9163: f64, t99: f64, t107: f64, t9232: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25177 = t9593 * t4144;
    let t25273 = t2698 * t159;
    let t25802 = t4147 * t4135;
    let t33127 = 1.0_f64 / t65 / t26;
    let t36227 = t99 * t9163;
    let t36415 = t107 * t9232;
    (t25177, t25273, t25802, t33127, t36227, t36415)
}
