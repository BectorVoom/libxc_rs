//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 931/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk931<F: Float>(t1378: F, t933: F, t1372: F, t331: F, t3595: F, t3591: F, t1333: F, t191: F, t205: F, t190: F, t212: F, t9821: F) -> (F, F, F, F, F, F) {
    let t10206 = t933 * t1378;
    let t10208 = t933 * t1372;
    let t10210 = t331 * t3595;
    let t10212 = t331 * t3591;
    let t10216 = t191 / t205 / t1333;
    let t10225 = F::new(0.10864197530864197) * t190 * t9821 * t212;
    (t10206, t10208, t10210, t10212, t10216, t10225)
}
