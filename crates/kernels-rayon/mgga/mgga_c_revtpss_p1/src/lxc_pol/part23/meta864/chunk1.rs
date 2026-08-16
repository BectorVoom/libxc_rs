//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2757/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2757(t22449: f64, t2435: f64, t136: f64, t2457: f64, t6918: f64, t9674: f64, t13999: f64, t22146: f64, t22145: f64, t48863: f64, t49137: f64, t124: f64, t6861: f64) -> (f64, f64, f64, f64, f64) {
    let t73707 = t2435 * t22449;
    let t73712 = t9674 * t6918 * t136 * t2457;
    let t73726 = t13999 * t22146;
    let t73729 = t49137 * t48863 * t22145;
    let t73731 = t124 * t6861;
    (t73707, t73712, t73726, t73729, t73731)
}
