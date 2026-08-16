//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 878/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk878(t1181: f64, t12816: f64, t604: f64, t7493: f64, t7685: f64, t957: f64, t2028: f64, t7599: f64, t2048: f64, t2052: f64, t7600: f64, t154: f64, t360: f64, t7322: f64, t7326: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30297 = t7493 * t1181 * t604 * t12816;
    let t30301 = t7685 * t957;
    let t30307 = t7599 * t2028;
    let t30308 = t30307 * t2048;
    let t30310 = t7600 * t2052;
    let t30314 = t7322 * t154 * t360 * t7326;
    (t30297, t30301, t30307, t30308, t30310, t30314)
}
