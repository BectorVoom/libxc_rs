//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 697/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk697(t8192: f64, t10976: f64, t10981: f64, t10985: f64, t10990: f64, t10993: f64, t10996: f64, t11000: f64, t11005: f64, t11010: f64, t11015: f64, t11019: f64, t11022: f64, t11024: f64, t11026: f64, t11027: f64, t7778: f64, t7782: f64, t7820: f64, t7822: f64) -> f64 {
    let t11031 = 4.0_f64 / 27.0_f64 * t8192;
    let t11032 = -t7822 / 27.0_f64 + 2.0_f64 / 27.0_f64 * t10976 + t10981 / 9.0_f64 + t10985 / 18.0_f64 + t10990 / 27.0_f64 - t10993 + 2.0_f64 / 9.0_f64 * t10996 + t11000 / 9.0_f64 + 4.0_f64 / 9.0_f64 * t11005 - 5.0_f64 / 81.0_f64 * t11010 - 4.0_f64 / 27.0_f64 * t11015 + t11019 / 18.0_f64 - t11022 - t11024 + t11026 - t11027 + t7778 / 54.0_f64 + t7782 / 81.0_f64 - t7820 / 27.0_f64 - t11031;
    t11032
}
