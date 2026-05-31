//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 653/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk653<F: Float>(t1105: F, t654: F, t687: F, t2799: F, t286: F, t2801: F, t1100: F, t637: F, t246: F, t394: F, t245: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3939 = t1105 * t654;
    let t3941 = t1105 * t687;
    let t3942 = F::cast_from(36.0_f64) * t3941;
    let t3944 = F::cast_from(24.0_f64) * t2799 * t286;
    let t3945 = t2801 * t286;
    let t3946 = F::cast_from(144.0_f64) * t3945;
    let t3947 = t637 * t1100;
    let t3948 = t3947 * t286;
    let t3949 = F::cast_from(240.0_f64) * t3948;
    let t3951 = F::cast_from(1.0_f64) / t246 / t394;
    let t3952 = t245 * t3951;
    let t3954 = F::cast_from(120.0_f64) * t3952 * t286;
    (t3939, t3941, t3942, t3944, t3945, t3946, t3947, t3948, t3949, t3951, t3952, t3954)
}
