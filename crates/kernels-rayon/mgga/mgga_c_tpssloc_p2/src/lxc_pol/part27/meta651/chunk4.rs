//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2268/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2268(t4017: f64, t645: f64, t72: f64, t46104: f64, t6489: f64, t12571: f64, t22522: f64, t26083: f64, t9239: f64, t1433: f64, t2241: f64, t22519: f64, t22523: f64, t22527: f64, t22531: f64, t22544: f64, t22546: f64, t22554: f64, t26021: f64, t26025: f64, t26051: f64, t26090: f64, t6490: f64, t6492: f64, t6495: f64, t7446: f64) -> f64 {
    let t90177 = t72 * t4017 * t645;
    let t90182 = t46104 * t6489;
    let t90185 = t12571 * t22522;
    let t90192 = t9239 * t26083;
    let t90196 = t72 * t1433 * t2241;
    let t90199 = 5.0_f64 / 3.0_f64 * t22554 * t26090 + 2.0_f64 / 3.0_f64 * t22519 * t7446 + 5.0_f64 / 3.0_f64 * t22523 * t26090 + 2.0_f64 / 3.0_f64 * t6495 * t26021 + 5.0_f64 / 3.0_f64 * t6490 * t90177 + 2.0_f64 / 3.0_f64 * t6495 * t26025 + 5.0_f64 / 3.0_f64 * t90182 * t6492 + 5.0_f64 / 3.0_f64 * t90185 * t6492 + 5.0_f64 / 3.0_f64 * t26051 * t22527 + 5.0_f64 / 6.0_f64 * t26051 * t22531 - 5.0_f64 * t90192 * t22546 - 5.0_f64 * t22544 * t90196;
    t90199
}
