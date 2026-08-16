//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2684/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2684(t39532: f64, t19572: f64, t67: f64, t758: f64, t39540: f64, t54428: f64, t16018: f64, t16490: f64, t193: f64, t19924: f64, t20093: f64, t3918: f64, t3919: f64, t39539: f64, t39549: f64, t39563: f64, t5122: f64, t5126: f64, t55224: f64, t6347: f64) -> (f64, f64, f64, f64, f64) {
    let t56372 = 0.70178683471615754484e1_f64 * t39532;
    let t56374 = t19572 * t67 * t758;
    let t56375 = 0.36622894612013090108e-3_f64 * t56374;
    let t56381 = 0.11696447245269292414e1_f64 * t39540;
    let t56388 = 24.0_f64 * t54428;
    let t56389 = 6.0_f64 * t16018 * t3918 * t5122 + 6.0_f64 * t16490 * t193 * t6347 + 24.0_f64 * t19924 * t3919 * t5126 + 12.0_f64 * t20093 * t55224 + t39539 + t39549 + t39563 - t56372 - t56375 - t56381 - t56388;
    (t56372, t56375, t56381, t56388, t56389)
}
