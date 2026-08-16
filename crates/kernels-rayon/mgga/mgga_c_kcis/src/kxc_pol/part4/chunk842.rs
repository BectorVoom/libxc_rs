//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 842/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk842(t3794: f64, t3795: f64, t5469: f64, t5472: f64, t5475: f64, t5479: f64) -> f64 {
    let t5481 = t3794 + t3795 / 9.0_f64 + t5469 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t5472 + 2.0_f64 / 3.0_f64 * t5475 + 2.0_f64 / 3.0_f64 * t5479;
    t5481
}
