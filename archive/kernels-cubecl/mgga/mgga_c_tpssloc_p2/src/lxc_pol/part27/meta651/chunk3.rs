//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2267/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2267<F: Float>(t1410: F, t9239: F, t2241: F, t72: F, t7431: F, t12648: F, t605: F, t12652: F, t12661: F, t1865: F, t26009: F, t26070: F, t26073: F, t26076: F, t6506: F, t6510: F, t83719: F, t83827: F, t83830: F) -> F {
    let t90137 = t9239 * t1410;
    let t90141 = t72 * t7431 * t2241;
    let t90150 = t605 * t12648;
    let t90153 = t605 * t12652;
    let t90160 = t605 * t12661;
    let t90167 = F::cast_from(10.0_f64) * t90137 * t83719 + F::cast_from(35.0_f64) * t83830 * t90141 - F::cast_from(10.0_f64) * t83827 * t26009 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26070 * t6506 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26070 * t6510 + t90150 * t1865 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t90153 * t1865 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26073 * t6506 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26073 * t6510 + t90160 * t1865 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26076 * t6506 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26076 * t6510;
    t90167
}
