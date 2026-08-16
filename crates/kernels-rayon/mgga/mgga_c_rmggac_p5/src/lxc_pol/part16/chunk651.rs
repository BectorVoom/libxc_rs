//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 651/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk651(t884: f64, t9302: f64, t530: f64, t8048: f64, t2205: f64, t2868: f64, t1624: f64, t699: f64, t1550: f64, t1627: f64, t903: f64, t2211: f64, t8377: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9303 = t884 * t9302;
    let t9310 = t530 * t8048;
    let t9312 = t2868 * t2205;
    let t9315 = t699 * t1624;
    let t9316 = t1550 * t9315;
    let t9318 = t699 * t1627;
    let t9319 = t903 * t9318;
    let t9321 = t2211 * t8377;
    (t9303, t9310, t9312, t9315, t9316, t9318, t9319, t9321)
}
