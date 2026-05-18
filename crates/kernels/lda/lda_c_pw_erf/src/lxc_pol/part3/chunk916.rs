//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 916/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk916<F: Float>(t1325: F, t3398: F, t5237: F, t3883: F, t529: F, t3403: F, t519: F, t3416: F, t3855: F, t4072: F, t518: F, t1251: F, t177: F, t191: F) -> (F, F, F, F, F, F) {
    let t9721 = t1325 * t5237 * t3398;
    let t9723 = t3883 * t529;
    let t9725 = t519 * t9723 * t3403;
    let t9737 = t3416 * t3855;
    let t9752 = t4072 * t518;
    let t9761 = t191 / t177 / t1251;
    (t9721, t9723, t9725, t9737, t9752, t9761)
}
