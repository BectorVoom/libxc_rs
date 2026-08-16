//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2268/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2268<F: Float>(t4017: F, t645: F, t72: F, t46104: F, t6489: F, t12571: F, t22522: F, t26083: F, t9239: F, t1433: F, t2241: F, t22519: F, t22523: F, t22527: F, t22531: F, t22544: F, t22546: F, t22554: F, t26021: F, t26025: F, t26051: F, t26090: F, t6490: F, t6492: F, t6495: F, t7446: F) -> F {
    let t90177 = t72 * t4017 * t645;
    let t90182 = t46104 * t6489;
    let t90185 = t12571 * t22522;
    let t90192 = t9239 * t26083;
    let t90196 = t72 * t1433 * t2241;
    let t90199 = F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t22554 * t26090 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t22519 * t7446 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t22523 * t26090 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6495 * t26021 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t6490 * t90177 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6495 * t26025 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t90182 * t6492 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t90185 * t6492 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t26051 * t22527 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t26051 * t22531 - F::cast_from(5.0_f64) * t90192 * t22546 - F::cast_from(5.0_f64) * t22544 * t90196;
    t90199
}
