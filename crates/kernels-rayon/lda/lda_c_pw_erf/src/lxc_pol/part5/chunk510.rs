//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 510/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk510(t2146: f64, t826: f64, t2337: f64, t575: f64, t574: f64, t571: f64, t1486: f64, t2334: f64, t1485: f64, t2171: f64, t799: f64, t2329: f64, t523: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2538 = 8.0_f64 / 45.0_f64 * t2146 * t826;
    let t2539 = t575 * t2337;
    let t2540 = t574 * t2539;
    let t2542 = 4.0_f64 / 45.0_f64 * t571 * t2540;
    let t2543 = t1486 * t2334;
    let t2544 = t1485 * t2543;
    let t2546 = 4.0_f64 / 27.0_f64 * t571 * t2544;
    let t2548 = 8.0_f64 / 45.0_f64 * t2171 * t799;
    let t2549 = t523 * t2329;
    (t2538, t2539, t2540, t2542, t2543, t2544, t2546, t2548, t2549)
}
