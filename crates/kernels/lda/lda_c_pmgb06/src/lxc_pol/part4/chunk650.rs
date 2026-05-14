//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 650/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk650<F: Float>(t1105: F, t654: F, t687: F, t2799: F, t286: F, t2801: F, t1100: F, t637: F, t246: F, t394: F, t245: F, t1108: F, t110: F, t980: F, t1121: F, t410: F, t698: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3939 = t1105 * t654;
    let t3941 = t1105 * t687;
    let t3944 = 24.0 * t2799 * t286;
    let t3945 = t2801 * t286;
    let t3947 = t637 * t1100;
    let t3948 = t3947 * t286;
    let t3951 = 1.0 / t246 / t394;
    let t3952 = t245 * t3951;
    let t3954 = 120.0 * t3952 * t286;
    let t3955 = t1108 * t687;
    let t3957 = t110 * t980;
    let t3959 = 0.03253074390090522 * t1121 * t3957;
    let t3960 = t410 * t698;
    (t3939, t3941, t3944, t3945, t3947, t3948, t3951, t3952, t3954, t3955, t3957, t3959, t3960)
}
