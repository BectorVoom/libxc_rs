//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 654/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk654(t2463: f64, t2604: f64, t1632: f64, t699: f64, t903: f64, t1635: f64, t1364: f64, t2211: f64, t5898: f64, t884: f64, t1562: f64, t2265: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9396 = t2604 * t2463;
    let t9399 = t699 * t1632;
    let t9400 = t903 * t9399;
    let t9402 = t699 * t1635;
    let t9403 = t1364 * t9402;
    let t9405 = t2211 * t5898;
    let t9406 = t884 * t9405;
    let t9408 = t1562 * t2265;
    (t9396, t9399, t9400, t9402, t9403, t9405, t9406, t9408)
}
