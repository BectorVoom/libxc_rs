//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 812/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk812<F: Float>(t1371: F, t7414: F, t1943: F, t2337: F, t1351: F, t7365: F, t589: F, t1948: F, t1349: F, t11: F, t25: F, t3600: F, t3639: F, t4657: F, t5024: F, t7405: F, t7409: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7415 = t1371 * t7414;
    let t7418 = t1943 * t2337;
    let t7419 = t1371 * t7418;
    let t7422 = t1351 * t7365;
    let t7423 = t589 * t7422;
    let t7426 = t1948 * t2337;
    let t7427 = t589 * t7426;
    let t7430 = t1349 * t7414;
    let t7431 = t11 * t7430;
    let t7433 = -F::cast_from(0.006666666666666667_f64) * t25 * t7405 - F::cast_from(0.002962962962962963_f64) * t25 * t7409 - t3600 - F::cast_from(0.022222222222222223_f64) * t5024 - F::cast_from(0.047988888888888886_f64) * t4657 - t3639 + F::cast_from(0.013333333333333334_f64) * t25 * t7415 - F::cast_from(0.006666666666666667_f64) * t25 * t7419 - F::cast_from(0.04_f64) * t25 * t7423 + F::cast_from(0.04_f64) * t25 * t7427 + F::cast_from(0.14396666666666666_f64) * t7431;
    (t7415, t7418, t7419, t7422, t7423, t7426, t7427, t7430, t7431, t7433)
}
