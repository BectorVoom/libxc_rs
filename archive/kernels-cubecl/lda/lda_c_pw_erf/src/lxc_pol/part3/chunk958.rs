//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 958/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk958<F: Float>(t10953: F, t169: F, t289: F, t3196: F, t462: F, t1089: F, t39: F, t343: F, t678: F, t2817: F, t5: F, t168: F, t286: F) -> (F, F, F, F, F, F) {
    let t10956 = F::cast_from(0.031835665774679375_f64) * t169 * t289 * t10953;
    let t10957 = t462 * t3196;
    let t10961 = t39 * t1089;
    let t10963 = t343 * t678;
    let t10967 = t5 * t2817;
    let t10970 = F::cast_from(0.9106331049773876_f64) * t168 * t10967 * t286;
    (t10956, t10957, t10961, t10963, t10967, t10970)
}
