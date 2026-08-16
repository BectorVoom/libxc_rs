//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3084/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3084(t43748: f64, t50903: f64, t50905: f64, t50907: f64, t50919: f64, t50921: f64, t50948: f64, t50950: f64, t63327: f64, t63330: f64, t63332: f64, t63334: f64, t63336: f64) -> f64 {
    let t63967 = 8.0_f64 * t63327 - 16.0_f64 / 3.0_f64 * t63330 - 8.0_f64 / 81.0_f64 * t63332 + 4.0_f64 / 27.0_f64 * t63334 - 2.0_f64 / 9.0_f64 * t63336 - 8.0_f64 / 81.0_f64 * t43748 - 8.0_f64 / 9.0_f64 * t50903 - 4.0_f64 / 9.0_f64 * t50905 - 4.0_f64 / 3.0_f64 * t50907 - 32.0_f64 / 81.0_f64 * t50919 - 20.0_f64 / 81.0_f64 * t50921 + 32.0_f64 / 27.0_f64 * t50948 + 8.0_f64 / 27.0_f64 * t50950;
    t63967
}
