//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 992/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk992(t2538: f64, t26416: f64, t7630: f64, t826: f64, t7655: f64, t898: f64, t2165: f64, t2772: f64, t874: f64, t9194: f64, t2157: f64, t710: f64, t7603: f64, t86: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26417 = t2538 * t26416;
    let t26418 = 2.0_f64 * t26417;
    let t26419 = t7630 * t826;
    let t26420 = t2538 * t26419;
    let t26421 = 4.0_f64 * t26420;
    let t26422 = t7655 * t898;
    let t26425 = t2165 * t2772;
    let t26430 = t874 * t9194;
    let t26431 = t26430 * t2157;
    let t26434 = t86 * t710 * t7603;
    (t26417, t26418, t26419, t26420, t26421, t26422, t26425, t26430, t26431, t26434)
}
