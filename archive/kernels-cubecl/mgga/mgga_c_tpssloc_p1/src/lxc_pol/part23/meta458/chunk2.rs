//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1338/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1338<F: Float>(t41741: F, t47787: F, t59657: F, t68442: F, t76574: F, t76578: F, t76583: F, t76587: F, t76591: F, t76595: F, t76599: F, t20217: F, t4337: F) -> (F, F) {
    let t76602 = F::cast_from(0.38456790123456790123e-1_f64) * t47787 - F::cast_from(0.27469135802469135803e-1_f64) * t76574 - F::cast_from(0.92708333333333333333e-2_f64) * t76578 - F::cast_from(0.16481481481481481482e-1_f64) * t59657 + F::cast_from(0.12361111111111111111e0_f64) * t76583 - F::cast_from(0.61805555555555555555e-1_f64) * t76587 - F::cast_from(0.22249999999999999999e0_f64) * t76591 + F::cast_from(0.22249999999999999999e0_f64) * t76595 - F::cast_from(0.18541666666666666666e-1_f64) * t76599 + t41741 + F::cast_from(0.74166666666666666668e-1_f64) * t68442;
    let t76608 = t4337 * t20217;
    (t76602, t76608)
}
