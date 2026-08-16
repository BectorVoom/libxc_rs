//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1040/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1040(t2169: f64, t26406: f64, t2533: f64, t7630: f64, t2161: f64, t2770: f64, t2153: f64, t2626: f64, t2538: f64, t826: f64, t7655: f64, t898: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26407 = t2169 * t26406;
    let t26408 = t26407 / 16.0_f64;
    let t26409 = t2533 * t7630;
    let t26410 = 2.0_f64 * t26409;
    let t26411 = t2161 * t2770;
    let t26416 = t2153 * t2626;
    let t26417 = t2538 * t26416;
    let t26418 = 2.0_f64 * t26417;
    let t26419 = t7630 * t826;
    let t26420 = t2538 * t26419;
    let t26421 = 4.0_f64 * t26420;
    let t26422 = t7655 * t898;
    (t26408, t26409, t26410, t26411, t26416, t26417, t26418, t26419, t26420, t26421, t26422)
}
