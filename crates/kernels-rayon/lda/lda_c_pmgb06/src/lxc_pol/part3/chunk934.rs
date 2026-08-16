//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 934/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk934(t1186: f64, t2847: f64, t421: f64, t1354: f64, t2822: f64, t2841: f64, t4240: f64, t4298: f64, t10644: f64, t118: f64, t2778: f64, t415: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10834 = t2847 * t1186 * t421;
    let t10838 = 0.013871971944573394_f64 * t2822 * t2841 * t1354;
    let t10840 = 0.12408369628826103_f64 * t4240 * t421;
    let t10843 = 0.02267957317922317_f64 * t4298 * t1354;
    let t10844 = t10644 * t118;
    let t10847 = 0.0004746123948660562_f64 * t2778 * t415;
    (t10834, t10838, t10840, t10843, t10844, t10847)
}
