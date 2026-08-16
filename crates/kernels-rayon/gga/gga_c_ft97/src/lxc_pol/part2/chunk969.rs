//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 969/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk969(t15051: f64, t2666: f64, t10559: f64, t10584: f64, t10586: f64, t10589: f64, t10591: f64, t10594: f64, t10595: f64, t10617: f64, t10619: f64, t13682: f64, t13688: f64, t15011: f64, t15014: f64, t15015: f64, t15018: f64, t15022: f64, t15025: f64, t15028: f64, t15039: f64, t15044: f64, t15048: f64, t462: f64) -> f64 {
    let t15052 = t15051 * t2666;
    let t15055 = -4.0_f64 / 9.0_f64 * t15011 + t15014 - 22.0_f64 / 9.0_f64 * t15015 - 6.0_f64 * t462 * t15018 + 2.0_f64 * t462 * t15022 - 4.0_f64 / 27.0_f64 * t15025 - t15028 - 2.0_f64 / 9.0_f64 * t10617 + t10559 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t10584 - 8.0_f64 / 9.0_f64 * t10595 - 8.0_f64 / 27.0_f64 * t10586 + t10589 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t10591 - 2.0_f64 / 9.0_f64 * t10619 + 4.0_f64 * t462 * t15039 + 4.0_f64 / 9.0_f64 * t13682 * t15044 - 4.0_f64 / 3.0_f64 * t13688 * t15048 - 4.0_f64 / 3.0_f64 * t13688 * t15052 - t10594;
    t15055
}
