//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 814/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk814(t3317: f64, t3319: f64, t3335: f64, t3342: f64, t3384: f64, t3388: f64, t3393: f64, t3946: f64, t3949: f64, t3950: f64, t3951: f64, t7851: f64, t7855: f64) -> f64 {
    let t8202 = 6.0_f64 * t7851 + 6.0_f64 * t7855 - 0.505765839233979_f64 * t3335 - 0.337177226155986_f64 * t3342 + 12.0_f64 * t3384 + 12.0_f64 * t3388 - 12.0_f64 * t3393 + t3946 + t3949 + t3950 - t3951 + 0.505765839233979_f64 * t3317 + 0.505765839233979_f64 * t3319;
    t8202
}
