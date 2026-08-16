//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 496/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk496(t2026: f64, t2027: f64, t2029: f64, t2030: f64, t2044: f64, t2047: f64, t2058: f64, t2060: f64, t2108: f64, t2110: f64, t2745: f64, t2749: f64, t2753: f64, t2783: f64, t453: f64, t472: f64) -> f64 {
    let t2791 = -t2026 - t2027 - t2029 - t2030 - t472 * t2783 / 6.0_f64 + t453 * t2783 / 6.0_f64 + t2044 - t2047 + t2058 + 0.037002892246025966_f64 * t2745 - 0.037002892246025966_f64 * t2749 - 0.14975624337724558_f64 * t2753 + t2060 - t2108 + t2110;
    t2791
}
