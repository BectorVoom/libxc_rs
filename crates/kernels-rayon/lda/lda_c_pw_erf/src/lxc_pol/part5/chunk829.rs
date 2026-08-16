//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 829/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk829(t4049: f64, t7612: f64, t571: f64, t2171: f64, t2550: f64, t2554: f64, t523: f64, t7360: f64, t522: f64, t519: f64, t3894: f64, t7354: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7613 = t4049 * t7612;
    let t7615 = 32.0_f64 / 81.0_f64 * t571 * t7613;
    let t7617 = 4.0_f64 / 15.0_f64 * t2171 * t2550;
    let t7619 = 4.0_f64 / 9.0_f64 * t2171 * t2554;
    let t7620 = t523 * t7360;
    let t7621 = t522 * t7620;
    let t7623 = 4.0_f64 / 45.0_f64 * t519 * t7621;
    let t7624 = t3894 * t7354;
    (t7613, t7615, t7617, t7619, t7620, t7621, t7623, t7624)
}
