//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 857/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk857<F: Float>(t455: F, t9148: F, t1568: F, t2765: F, t440: F, t142: F, t450: F, t2766: F, t1089: F, t1191: F, t169: F, t301: F, t3365: F, t405: F, t1554: F, t3327: F) -> (F, F, F, F, F, F, F) {
    let t10823 = t455 * t9148;
    let t10829 = t2765 * t440 * t1568;
    let t10832 = t450 * t142;
    let t10833 = t10832 * t2766;
    let t10838 = t169 * t1191 * t1089 * t301;
    let t10843 = t405 * t3365;
    let t10847 = t1554 * t142 * t3327;
    (t10823, t10829, t10832, t10833, t10838, t10843, t10847)
}
