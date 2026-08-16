//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 938/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk938<F: Float>(t10463: F, t1325: F, t1328: F, t3783: F, t529: F, t1314: F, t519: F, t3393: F, t3859: F, t3454: F, t518: F, t4025: F) -> (F, F, F, F, F, F) {
    let t10465 = t1325 * t10463 * t1328;
    let t10467 = t3783 * t529;
    let t10469 = t519 * t10467 * t1314;
    let t10472 = t1325 * t3859 * t3393;
    let t10474 = t3454 * t518;
    let t10488 = t4025 * t518;
    (t10465, t10467, t10469, t10472, t10474, t10488)
}
