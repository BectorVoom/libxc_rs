//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 892/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk892<F: Float>(t1180: F, t209: F, t211: F, t4088: F, t591: F, t4094: F, t4096: F, t4111: F, t4103: F, t574: F, t581: F, t3050: F, t432: F) -> (F, F, F, F, F, F, F) {
    let t9417 = F::cast_from(56.0_f64) / F::cast_from(243.0_f64) * t209 * t211 * t1180;
    let t9418 = t4088 * t591;
    let t9422 = t4094 * t591;
    let t9424 = t4096 * t4111;
    let t9426 = t574 * t4103;
    let t9429 = F::cast_from(32.0_f64) / F::cast_from(81.0_f64) * t581 * t4103;
    let t9434 = t432 * t3050;
    (t9417, t9418, t9422, t9424, t9426, t9429, t9434)
}
