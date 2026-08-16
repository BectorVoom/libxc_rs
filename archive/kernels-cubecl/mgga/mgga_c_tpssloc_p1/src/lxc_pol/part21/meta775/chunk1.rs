//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2684/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2684<F: Float>(t39532: F, t19572: F, t67: F, t758: F, t39540: F, t54428: F, t16018: F, t16490: F, t193: F, t19924: F, t20093: F, t3918: F, t3919: F, t39539: F, t39549: F, t39563: F, t5122: F, t5126: F, t55224: F, t6347: F) -> (F, F, F, F, F) {
    let t56372 = F::cast_from(0.70178683471615754484e1_f64) * t39532;
    let t56374 = t19572 * t67 * t758;
    let t56375 = F::cast_from(0.36622894612013090108e-3_f64) * t56374;
    let t56381 = F::cast_from(0.11696447245269292414e1_f64) * t39540;
    let t56388 = F::cast_from(24.0_f64) * t54428;
    let t56389 = F::cast_from(6.0_f64) * t16018 * t3918 * t5122 + F::cast_from(6.0_f64) * t16490 * t193 * t6347 + F::cast_from(24.0_f64) * t19924 * t3919 * t5126 + F::cast_from(12.0_f64) * t20093 * t55224 + t39539 + t39549 + t39563 - t56372 - t56375 - t56381 - t56388;
    (t56372, t56375, t56381, t56388, t56389)
}
