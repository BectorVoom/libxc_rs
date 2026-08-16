//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 826/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk826(t3317: f64, t3335: f64, t3342: f64, t3599: f64, t3601: f64, t7801: f64, t7805: f64, t7809: f64, t7811: f64, t7814: f64, t7817: f64, t7834: f64) -> f64 {
    let t8360 = -4.0_f64 / 3.0_f64 * t7801 - 2.0_f64 * t7805 - 2.0_f64 * t7809 - 2.0_f64 * t7811 - 2.0_f64 * t7814 - 2.0_f64 * t7817 - 2.0_f64 * t7834 - 2.0_f64 * t3335 - 4.0_f64 / 3.0_f64 * t3342 + t3599 - t3601 + 2.0_f64 * t3317;
    t8360
}
