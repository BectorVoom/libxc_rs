//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3065/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3065(t43748: f64, t50903: f64, t50905: f64, t50907: f64, t50919: f64, t50921: f64, t50948: f64, t50950: f64, t63327: f64, t63330: f64, t63332: f64, t63334: f64, t63336: f64) -> f64 {
    let t63679 = 0.2225e0_f64 * t63327 - 0.14833333333333333333e0_f64 * t63330 - 0.27469135802469135803e-2_f64 * t63332 + 0.41203703703703703704e-2_f64 * t63334 - 0.61805555555555555556e-2_f64 * t63336 - 0.27469135802469135802e-2_f64 * t43748 - 0.24722222222222222222e-1_f64 * t50903 - 0.12361111111111111111e-1_f64 * t50905 - 0.37083333333333333333e-1_f64 * t50907 - 0.10987654320987654321e-1_f64 * t50919 - 0.68672839506172839507e-2_f64 * t50921 + 0.32962962962962962963e-1_f64 * t50948 + 0.82407407407407407408e-2_f64 * t50950;
    t63679
}
