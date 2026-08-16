//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1059/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1059(t1507: f64, t16540: f64, t4911: f64, t555: f64, t1517: f64, t1527: f64, t4999: f64, t5002: f64, t1625: f64, t1661: f64, t83: f64, t1639: f64, t5155: f64) -> (f64, f64, f64, f64) {
    let t16544 = 0.6233709278045326953e3_f64 * t555 * t4911 * t16540 * t1507;
    let t16548 = 0.3103560775156404018e4_f64 * t4999 * t1517 * t5002 * t1527;
    let t16550 = t83 * t1661 * t1625;
    let t16552 = t5155 * t1639;
    (t16544, t16548, t16550, t16552)
}
