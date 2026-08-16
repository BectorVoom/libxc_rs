//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1151/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1151(t52212: f64, t52916: f64, t66902: f64, t66905: f64, t66934: f64, t66945: f64, t67420: f64, t80685: f64, t80696: f64, t80759: f64, t80770: f64, t80772: f64, t88198: f64, t88201: f64, t88213: f64) -> f64 {
    let t89513 = 8.0_f64 / 3.0_f64 * t80685 - 8.0_f64 / 9.0_f64 * t66902 + 16.0_f64 / 9.0_f64 * t66905 + 8.0_f64 / 9.0_f64 * t88198 - 8.0_f64 / 3.0_f64 * t88201 + 8.0_f64 / 9.0_f64 * t80696 + 16.0_f64 / 27.0_f64 * t66934 - 8.0_f64 / 27.0_f64 * t66945 + 112.0_f64 / 243.0_f64 * t52212 + 112.0_f64 / 81.0_f64 * t52916 - 16.0_f64 / 27.0_f64 * t80759 - 16.0_f64 / 81.0_f64 * t67420 + 8.0_f64 / 27.0_f64 * t80770 - 8.0_f64 / 27.0_f64 * t80772 + 3.0_f64 / 4.0_f64 * t88213;
    t89513
}
