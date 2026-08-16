//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1366/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1366(t22463: f64, t5905: f64, t17396: f64, t492: f64, t6029: f64, t21038: f64, t5904: f64, t5903: f64, t1517: f64, t167: f64, t5987: f64, t531: f64, t7190: f64) -> (f64, f64, f64, f64, f64) {
    let t22464 = t22463 * t5905;
    let t22466 = t17396 * t492;
    let t22467 = t22466 * t6029;
    let t22470 = t5904 * t21038;
    let t22471 = t5903 * t22470;
    let t22498 = t1517 * t5987 * t167;
    let t22503 = t7190 * t531;
    (t22464, t22467, t22471, t22498, t22503)
}
