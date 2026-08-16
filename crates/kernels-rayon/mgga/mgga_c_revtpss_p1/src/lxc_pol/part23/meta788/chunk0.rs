//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2601/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2601(t14472: f64, t1580: f64, t2439: f64, t2444: f64, t6049: f64, t689: f64, t136: f64, t2457: f64, t41011: f64, t6048: f64, t10504: f64, t6071: f64) -> (f64, f64, f64, f64) {
    let t61400 = t2439 * t14472 * t1580;
    let t61403 = t689 * t2444 * t6049;
    let t61407 = t41011 * t6048 * t136 * t2457;
    let t61411 = t10504 * t6071 * t136 * t2457;
    (t61400, t61403, t61407, t61411)
}
