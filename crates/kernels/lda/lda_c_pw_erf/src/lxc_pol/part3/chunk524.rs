//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 524/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk524<F: Float>(t169: F, t242: F, t2877: F, t465: F, t717: F, t1098: F, t632: F, t1112: F, t299: F, t1102: F, t1143: F, t699: F, t171: F, t2853: F, t1113: F, t703: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2880 = 0.5188034422540342 * t169 * t2877 * t242;
    let t2881 = t717 * t465;
    let t2883 = t169 * t2881 * t242;
    let t2887 = 0.42447554366239165 * t169 * t1098 * t632;
    let t2888 = t299 * t1112;
    let t2890 = t169 * t2888 * t242;
    let t2893 = t169 * t1102 * t632;
    let t2897 = 0.15917832887339686 * t169 * t699 * t1143;
    let t2898 = t171 * t2853;
    let t2903 = t169 * t1113 * t632;
    let t2906 = t169 * t703 * t1143;
    (t2880, t2881, t2883, t2887, t2888, t2890, t2893, t2897, t2898, t2903, t2906)
}
