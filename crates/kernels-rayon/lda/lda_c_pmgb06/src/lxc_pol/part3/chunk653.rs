//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 653/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk653(t1105: f64, t654: f64, t687: f64, t2799: f64, t286: f64, t2801: f64, t1100: f64, t637: f64, t246: f64, t394: f64, t245: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3939 = t1105 * t654;
    let t3941 = t1105 * t687;
    let t3942 = 36.0_f64 * t3941;
    let t3944 = 24.0_f64 * t2799 * t286;
    let t3945 = t2801 * t286;
    let t3946 = 144.0_f64 * t3945;
    let t3947 = t637 * t1100;
    let t3948 = t3947 * t286;
    let t3949 = 240.0_f64 * t3948;
    let t3951 = 1.0_f64 / t246 / t394;
    let t3952 = t245 * t3951;
    let t3954 = 120.0_f64 * t3952 * t286;
    (t3939, t3941, t3942, t3944, t3945, t3946, t3947, t3948, t3949, t3951, t3952, t3954)
}
