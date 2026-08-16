//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 988/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk988(t40273: f64, t39761: f64, t39767: f64, t39772: f64, t39776: f64, t39781: f64, t39784: f64, t39788: f64, t39792: f64, t39796: f64, t40265: f64, t40270: f64, t40288: f64, t40292: f64) -> f64 {
    let t40567 = 56.0_f64 / 81.0_f64 * t40273;
    let t40570 = 2.0_f64 / 9.0_f64 * t39761 + 4.0_f64 / 3.0_f64 * t39767 + t39772 - 4.0_f64 / 3.0_f64 * t39776 - 40.0_f64 / 243.0_f64 * t39781 + 4.0_f64 / 9.0_f64 * t39784 + 4.0_f64 / 3.0_f64 * t39788 + t39792 / 3.0_f64 - t39796 / 9.0_f64 - t40265 / 6.0_f64 + 4.0_f64 * t40270 + t40567 - 6.0_f64 * t40288 - t40292 / 18.0_f64;
    t40570
}
