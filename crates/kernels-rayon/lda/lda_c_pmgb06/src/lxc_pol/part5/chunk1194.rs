//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1194/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1194(t1291: f64, t1296: f64, t21568: f64, t21583: f64, t21590: f64, t21595: f64, t2238: f64, t2255: f64, t2722: f64, t2730: f64, t3625: f64, t3632: f64, t378: f64, t384: f64, t7086: f64, t7334: f64, t7337: f64, t7351: f64, t787: f64, t8404: f64, t8413: f64) -> f64 {
    let t21599 = -3.0_f64 * t2238 * t7086 - 6.0_f64 * t8404 * t7334 + 24.0_f64 * t8413 * t7334 * t384 - 18.0_f64 * t3632 * t2722 * t2255 + 6.0_f64 * t3625 * t7337 - 18.0_f64 * t3632 * t7337 * t384 + 6.0_f64 * t1296 * t2255 * t2730 + 6.0_f64 * t1296 * t787 * t7086 - t1291 * t7351 + 2.0_f64 * t1296 * t7351 * t384 - t378 * (t21568 + t21583 + t21590 + t21595);
    t21599
}
