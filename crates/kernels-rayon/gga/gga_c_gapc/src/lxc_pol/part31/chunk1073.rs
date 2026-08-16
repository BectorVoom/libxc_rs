//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1073/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1073(t12633: f64, t12637: f64, t12644: f64, t12648: f64, t12042: f64, t12048: f64, t12049: f64, t12051: f64, t12057: f64, t12060: f64, t12287: f64, t12290: f64, t12293: f64, t12586: f64, t12588: f64, t12589: f64, t12623: f64, t2464: f64, t3914: f64, t884: f64) -> (f64, f64) {
    let t12650 = t12633 + t12637 + t12644 + t12648;
    let t12653 = -t12650 * t884 - t2464 * t3914 + t12042 - t12048 + t12049 - t12051 - t12057 - t12060 + t12287 - t12290 + t12293 - t12586 + t12588 + t12589 + t12623;
    (t12650, t12653)
}
