//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1029/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1029(t3712: f64, t5625: f64, t137: f64, t1403: f64, t442: f64, t5215: f64, t1: f64, t5700: f64, t19508: f64, t4867: f64, t144: f64, t5698: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19546 = t3712 * t5625;
    let t19586 = t1403 * t137;
    let t19588 = t5215 * t19586 * t442;
    let t19622 = t5700 * t1;
    let t19624 = t19508 * t19622 * t4867;
    let t19636 = t144 * t5698;
    (t19546, t19586, t19588, t19622, t19624, t19636)
}
