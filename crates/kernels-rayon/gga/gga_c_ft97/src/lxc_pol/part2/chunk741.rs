//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 741/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk741(t1882: f64, t3268: f64, t10992: f64, t10976: f64, t10981: f64, t10985: f64, t10990: f64, t10996: f64, t11000: f64, t11005: f64, t11010: f64, t11015: f64, t7822: f64) -> (f64, f64) {
    let t11632 = 4.0_f64 / 9.0_f64 * t1882 * t3268;
    let t11638 = 2.0_f64 / 27.0_f64 * t10992;
    let t11644 = -2.0_f64 / 27.0_f64 * t7822 + 4.0_f64 / 27.0_f64 * t10976 + 2.0_f64 / 9.0_f64 * t10981 + t10985 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t10990 - t11638 + 4.0_f64 / 9.0_f64 * t10996 + 2.0_f64 / 9.0_f64 * t11000 + 8.0_f64 / 9.0_f64 * t11005 - 10.0_f64 / 81.0_f64 * t11010 - 8.0_f64 / 27.0_f64 * t11015;
    (t11632, t11644)
}
