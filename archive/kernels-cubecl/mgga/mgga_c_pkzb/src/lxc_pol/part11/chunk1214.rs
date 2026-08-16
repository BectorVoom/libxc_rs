//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1214/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1214<F: Float>(t12: F, t10513: F, t10518: F, t1430: F, t17361: F, t1837: F, t2732: F, t28874: F, t28877: F, t28885: F, t439: F, t652: F, t7337: F, t7340: F, t8729: F, t9150: F, zeta_threshold: F) -> F {
    let t84 = t12 <= zeta_threshold;
    let t29813 = piecewise3::<F>(t84, F::cast_from(0.0_f64), F::cast_from(280.0_f64) / F::cast_from(81.0_f64) * t17361 * t10513 * t439 - F::cast_from(56.0_f64) / F::cast_from(9.0_f64) * t9150 * t1430 - F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t7337 * t28874 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t7340 * t28877 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2732 * t8729 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1837 * t10518 * t439 - t652 * t28885 / F::cast_from(3.0_f64));
    t29813
}
