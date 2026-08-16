//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 950/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk950(t9062: f64, t9066: f64, t9150: f64, t9152: f64, t9155: f64, t9158: f64, t9163: f64, t9166: f64, t9168: f64, t9170: f64, t9173: f64, t9176: f64, t9179: f64, t9182: f64) -> f64 {
    let t9311 = -t9062 / 8.0_f64 - 3.0_f64 / 4.0_f64 * t9066 + t9150 / 8.0_f64 - 3.0_f64 / 8.0_f64 * t9152 + 3.0_f64 / 32.0_f64 * t9155 + t9158 / 64.0_f64 + 15.0_f64 / 8.0_f64 * t9163 - 3.0_f64 / 2.0_f64 * t9166 - 3.0_f64 / 4.0_f64 * t9168 + 3.0_f64 / 64.0_f64 * t9170 + 3.0_f64 / 4.0_f64 * t9173 - t9176 / 64.0_f64 + 3.0_f64 / 8.0_f64 * t9179 - 3.0_f64 / 8.0_f64 * t9182;
    t9311
}
