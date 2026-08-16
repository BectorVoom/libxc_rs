//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 682/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk682<F: Float>(t1101: F, t654: F, t687: F, t594: F, t598: F, t1105: F, t2799: F, t286: F, t2801: F, t1100: F, t637: F, t246: F, t394: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3908 = t1101 * t654;
    let t3911 = F::cast_from(60.0_f64) * t1101 * t687;
    let t3912 = F::cast_from(1.0_f64) / t594;
    let t3922 = F::cast_from(1.0_f64) / t598;
    let t3939 = t1105 * t654;
    let t3941 = t1105 * t687;
    let t3944 = F::cast_from(24.0_f64) * t2799 * t286;
    let t3945 = t2801 * t286;
    let t3947 = t637 * t1100;
    let t3948 = t3947 * t286;
    let t3951 = F::cast_from(1.0_f64) / t246 / t394;
    (t3908, t3911, t3912, t3922, t3939, t3941, t3944, t3945, t3947, t3948, t3951)
}
