//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1111/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1111<F: Float>(t12083: F, t67: F, t758: F, t2505: F, t2527: F, t1294: F, t3691: F, t9905: F, t9892: F, t2368: F, t747: F, t9711: F) -> (F, F, F, F, F, F) {
    let t39334 = t12083 * t67 * t758;
    let t39335 = F::cast_from(0.73245789224026180216e-3_f64) * t39334;
    let t39336 = t2527 * t2505;
    let t39338 = F::cast_from(0.21053605041484726346e2_f64) * t1294 * t39336;
    let t39339 = t3691 * t9905;
    let t39340 = F::cast_from(0.14035736694323150897e2_f64) * t39339;
    let t39341 = t3691 * t9892;
    let t39342 = F::cast_from(0.20779030926817756511e3_f64) * t39341;
    let t39344 = t2368 * t9711 * t747;
    (t39335, t39336, t39338, t39340, t39342, t39344)
}
