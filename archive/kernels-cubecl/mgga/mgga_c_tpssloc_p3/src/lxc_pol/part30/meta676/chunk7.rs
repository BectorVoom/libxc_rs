//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2114/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2114<F: Float>(t26021: F, t26025: F, t26028: F, t26045: F, t26051: F, t26063: F, t26070: F, t26073: F, t26076: F, t27979: F, t6506: F, t6510: F, t7428: F, t7432: F, t7435: F, t7442: F, t7446: F, t90182: F, t90185: F) -> F {
    let t96605 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26070 * t7446 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26073 * t7446 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26076 * t7446 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7435 * t26021 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7435 * t26025 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t90182 * t7432 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t90185 * t7432 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t26051 * t26063 + t27979 * t6506 / F::cast_from(3.0_f64) + t27979 * t6510 / F::cast_from(3.0_f64) - t26028 * t7442 / F::cast_from(3.0_f64) - t7428 * t26045 / F::cast_from(3.0_f64);
    t96605
}
