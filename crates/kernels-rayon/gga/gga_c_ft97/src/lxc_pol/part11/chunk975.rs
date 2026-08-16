//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 975/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk975(t37357: f64, t40294: f64, t7761: f64, t89: f64, t39767: f64, t39772: f64, t39776: f64, t39781: f64, t39784: f64, t39788: f64, t39792: f64, t39796: f64, t40265: f64, t40270: f64, t40273: f64, t40283: f64, t40288: f64, t40292: f64) -> (f64, f64) {
    let t40297 = t89 * t7761 * t40294 * t37357;
    let t40299 = 8.0_f64 * t39767 + 6.0_f64 * t39772 - 8.0_f64 * t39776 - 80.0_f64 / 81.0_f64 * t39781 + 8.0_f64 / 3.0_f64 * t39784 + 8.0_f64 * t39788 + 2.0_f64 * t39792 - 2.0_f64 / 3.0_f64 * t39796 - t40265 + 24.0_f64 * t40270 + 112.0_f64 / 27.0_f64 * t40273 - 15.0_f64 / 16.0_f64 * t40283 - 36.0_f64 * t40288 - t40292 / 3.0_f64 + 40.0_f64 / 9.0_f64 * t40297;
    (t40297, t40299)
}
