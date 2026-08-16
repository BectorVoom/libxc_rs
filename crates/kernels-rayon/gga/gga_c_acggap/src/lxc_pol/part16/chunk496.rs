//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 496/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk496(t2671: f64, t2674: f64, t2677: f64, t2679: f64, t2683: f64, t2685: f64, t2687: f64, t2690: f64, t219: f64, t201: f64, t132: f64, t686: f64) -> (f64, f64) {
    let t2692 = -0.25319e1_f64 * t2671 + 0.16879333333333333333e1_f64 * t2674 - 0.19692555555555555555e1_f64 * t2677 - 0.93011851851851851854e0_f64 * t2679 + 0.13651666666666666667e0_f64 * t2683 - 0.27303333333333333333e0_f64 * t2685 - 0.3185388888888888889e0_f64 * t2687 - 0.36514074074074074075e0_f64 * t2690;
    let t2693 = t2692 * t219;
    let t2694 = t201 * t2693;
    let t2695 = 1.0_f64 * t2694;
    let t2696 = t132 * t686;
    (t2695, t2696)
}
