//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1442/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1442(t14472: f64, t1580: f64, t2439: f64, t136: f64, t2457: f64, t41011: f64, t6048: f64, t10504: f64, t6071: f64, t18317: f64, t2435: f64, t10815: f64, t6019: f64) -> (f64, f64, f64, f64, f64) {
    let t61400 = t2439 * t14472 * t1580;
    let t61407 = t41011 * t6048 * t136 * t2457;
    let t61411 = t10504 * t6071 * t136 * t2457;
    let t61448 = t2435 * t18317;
    let t61570 = t10815 * t6019;
    (t61400, t61407, t61411, t61448, t61570)
}
