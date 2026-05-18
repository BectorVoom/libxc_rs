//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 895/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk895<F: Float>(t411: F, t717: F, t732: F, t3257: F, t1124: F, t119: F, t1657: F, t8994: F, t1690: F, t415: F, t8916: F, t3273: F, t8920: F) -> (F, F, F, F, F, F, F) {
    let t8998 = t732 * t717 * t411;
    let t8999 = t3257 * t8998;
    let t9002 = t119 * t1124 * t411;
    let t9003 = t1657 * t9002;
    let t9005 = t1657 * t8994;
    let t9009 = t415 * t1690 * t8916;
    let t9011 = t3273 * t8920;
    (t8998, t8999, t9002, t9003, t9005, t9009, t9011)
}
