//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 350/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk350(t152: f64, t1552: f64, t172: f64, t129: f64, t19: f64, t464: f64, t1412: f64, t188: f64, t20: f64, t128: f64, t173: f64, t432: f64, t640: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1553 = t1552 * t152;
    let t1554 = t1553 * t172;
    let t1555 = t129 * t1554;
    let t1558 = t464 * t19;
    let t1559 = t1412 * t1558;
    let t1560 = t20 * t188;
    let t1561 = t1560 * t128;
    let t1562 = t1561 * t173;
    let t1565 = t432 * t640;
    (t1554, t1555, t1559, t1560, t1561, t1562, t1565)
}
