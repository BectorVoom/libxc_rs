//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 866/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk866<F: Float>(t1759: F, t4295: F, t1059: F, t2948: F, t2979: F, t402: F, t75: F, t390: F, t40: F, t3189: F, t344: F, t339: F) -> (F, F, F, F, F, F) {
    let t8301 = t1759 * t4295;
    let t8303 = t1059 * t2948;
    let t8306 = t2979 * t75 * t402;
    let t8309 = t40 * t2979 * t390;
    let t8311 = t344 * t3189;
    let t8313 = t339 * t3189;
    (t8301, t8303, t8306, t8309, t8311, t8313)
}
