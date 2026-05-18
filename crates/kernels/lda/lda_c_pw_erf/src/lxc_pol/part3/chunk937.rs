//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 937/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk937<F: Float>(t220: F, t4567: F, t211: F, t1524: F, t1529: F, t3802: F, t3850: F, t519: F, t1446: F, t3803: F, t197: F, t3783: F) -> (F, F, F, F, F, F) {
    let t10436 = t4567 * t220;
    let t10438 = F::new(112.0) / F::new(1215.0) * t211 * t10436;
    let t10439 = t1524 * t1529;
    let t10454 = t519 * t3802 * t3850;
    let t10456 = t1446 * t3803;
    let t10463 = t3783 * t197;
    (t10436, t10438, t10439, t10454, t10456, t10463)
}
