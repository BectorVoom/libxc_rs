//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2289/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2289(t15486: f64, t5024: f64, t15590: f64, t5018: f64, t15507: f64, t15548: f64, t13969: f64, t19057: f64, t3506: f64, t15438: f64, t15569: f64, t15608: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t66155 = t5024 * t15486;
    let t66159 = t15590 * t5018;
    let t66165 = t15507 * t15548;
    let t66241 = t3506 * t13969 * t19057;
    let t66255 = t15438 * t15548;
    let t66268 = t15569 * t15608;
    (t66155, t66159, t66165, t66241, t66255, t66268)
}
