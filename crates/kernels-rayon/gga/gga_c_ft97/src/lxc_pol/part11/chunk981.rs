//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 981/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk981(t1775: f64, t9225: f64, t9183: f64, t355: f64, t7368: f64, t2: f64, t9199: f64, t9193: f64, t525: f64, t7760: f64, t9218: f64, t2102: f64, t3499: f64, t3506: f64, t37315: f64, t37320: f64, t39719: f64, t39726: f64, t39735: f64, t39751: f64, t39755: f64, t39759: f64, t39765: f64, t462: f64, t9192: f64, t9217: f64) -> (f64, f64, f64) {
    let t40405 = t1775 * t9225;
    let t40413 = t1775 * t9183;
    let t40424 = t355 * t7368;
    let t40425 = t40424 * t2;
    let t40432 = t1775 * t9199;
    let t40434 = t1775 * t9193;
    let t40436 = t7760 * t525;
    let t40437 = t40436 * t2;
    let t40444 = t1775 * t9218;
    let t40446 = 40.0_f64 / 81.0_f64 * t40405 + 4.0_f64 / 3.0_f64 * t462 * t2102 * t39735 + 4.0_f64 / 3.0_f64 * t462 * t9192 * t39759 - 4.0_f64 / 3.0_f64 * t40413 - 4.0_f64 * t462 * t2102 * t39755 + 8.0_f64 * t462 * t2102 * t39719 - 12.0_f64 * t462 * t3506 * t37315 + 8.0_f64 * t462 * t40425 * t39751 + 8.0_f64 * t462 * t9217 * t39765 + 8.0_f64 / 9.0_f64 * t40432 - 8.0_f64 / 9.0_f64 * t40434 + 40.0_f64 / 27.0_f64 * t462 * t40437 * t39726 + 8.0_f64 * t462 * t3499 * t37320 + 8.0_f64 / 3.0_f64 * t40444;
    (t40424, t40436, t40446)
}
