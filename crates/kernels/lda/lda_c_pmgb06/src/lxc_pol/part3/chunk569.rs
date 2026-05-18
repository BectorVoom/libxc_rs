//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 569/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk569<F: Float>(t1439: F, t3010: F, t442: F, t439: F, t183: F, t2803: F, t1166: F, t539: F, t188: F, t1830: F, t2060: F, t83: F) -> (F, F, F, F, F, F, F, F) {
    let t3011 = t1439 * t3010;
    let t3012 = t442 * t3011;
    let t3014 = F::new(2.0) / F::new(15.0) * t439 * t3012;
    let t3015 = t2803 * t183;
    let t3018 = t1166 * t539;
    let t3019 = t3018 * t188;
    let t3023 = F::new(1.2833333333333334) * t1830 - F::new(20.0) / F::new(27.0) * t2060;
    let t3024 = t83 * t3023;
    (t3011, t3012, t3014, t3015, t3018, t3019, t3023, t3024)
}
