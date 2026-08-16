//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 352/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk352(t431: f64, t515: f64, t126: f64, t514: f64, t144: f64, t190: f64, t200: f64, t442: f64, t583: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1572 = t431 * t515;
    let t1573 = t1572 * t126;
    let t1574 = t514 * t1573;
    let t1575 = t190 * t144;
    let t1576 = t1575 * t200;
    let t1577 = t583 * t442;
    (t1572, t1573, t1574, t1575, t1576, t1577)
}
