//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 349/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk349(t1506: f64, t429: f64, t438: f64, t914: f64, t146: f64, t1497: f64, t455: f64, t1502: f64, t449: f64, t894: f64, t1514: f64, t464: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1527 = t429 * t1506;
    let t1528 = t1527 * t438;
    let t1529 = t914 * t1528;
    let t1533 = t146 * t455 * t1497;
    let t1536 = t914 * t1502;
    let t1539 = t449 * t1506;
    let t1540 = t1539 * t438;
    let t1541 = t894 * t1540;
    let t1544 = t464 * t1514;
    (t1528, t1529, t1533, t1536, t1540, t1541, t1544)
}
