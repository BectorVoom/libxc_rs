//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 524/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk524(t1587: f64, t552: f64, t321: f64, t6557: f64, t333: f64, t128: f64, t5840: f64, t305: f64, t1926: f64, t6444: f64, t6376: f64, t326: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6570 = t552 * t1587;
    let t6583 = t6557 * t321;
    let t6586 = t6557 * t333;
    let t6589 = t128 * t5840;
    let t6590 = t305 * t6589;
    let t6592 = t6444 * t1926;
    let t6598 = t128 * t6376;
    let t6599 = t326 * t6598;
    (t6570, t6583, t6586, t6590, t6592, t6599)
}
