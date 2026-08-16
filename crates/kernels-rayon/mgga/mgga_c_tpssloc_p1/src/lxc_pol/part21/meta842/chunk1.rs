//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3036/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3036(t43748: f64, t50903: f64, t50905: f64, t50907: f64, t50919: f64, t50921: f64, t50948: f64, t50950: f64, t63327: f64, t63330: f64, t63332: f64, t63334: f64, t63336: f64) -> f64 {
    let t63346 = 0.41096e0_f64 * t63327 - 0.27397333333333333332e0_f64 * t63330 - 0.50735802469135802467e-2_f64 * t63332 + 0.76103703703703703702e-2_f64 * t63334 - 0.11415555555555555555e-1_f64 * t63336 - 0.50735802469135802469e-2_f64 * t43748 - 0.4566222222222222222e-1_f64 * t50903 - 0.2283111111111111111e-1_f64 * t50905 - 0.6849333333333333333e-1_f64 * t50907 - 0.20294320987654320986e-1_f64 * t50919 - 0.12683950617283950617e-1_f64 * t50921 + 0.6088296296296296296e-1_f64 * t50948 + 0.1522074074074074074e-1_f64 * t50950;
    t63346
}
