//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 891/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk891<F: Float>(t3165: F, t338: F, t88: F, t1063: F, t35: F, t8327: F, t1035: F, t1064: F, t1039: F, t3128: F, t3130: F, t905: F, t935: F) -> (F, F, F, F, F, F) {
    let t8518 = t338 * t3165 * t88;
    let t8520 = t35 * t1063;
    let t8524 = t8327 * t88;
    let t8527 = F::cast_from(120.0_f64) * t1064 * t1035;
    let t8528 = t1064 * t1039;
    let t8533 = F::cast_from(3103.50088234237_f64) * t3128 * t935 * t3130 * t905;
    (t8518, t8520, t8524, t8527, t8528, t8533)
}
