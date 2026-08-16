//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1268/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1268(t1006: f64, t11223: f64, t1603: f64, t3639: f64, t4893: f64, t11257: f64, t4644: f64, t1265: f64, t1459: f64, t3649: f64, t3652: f64, t11182: f64, t11185: f64) -> (f64, f64, f64, f64, f64) {
    let t35628 = t1006 * t11223 * t1603;
    let t35631 = t1006 * t3639 * t4893;
    let t35634 = t11257 * t3639 * t4644;
    let t35638 = t3649 * t1265 * t1459 * t3652;
    let t35640 = t11182 * t11185;
    (t35628, t35631, t35634, t35638, t35640)
}
