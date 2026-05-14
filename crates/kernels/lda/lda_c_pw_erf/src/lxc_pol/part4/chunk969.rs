//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 969/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk969<F: Float>(t1063: F, t274: F, t169: F, t242: F, t2818: F, t1113: F, t1143: F, t2929: F, t703: F, t699: F, t2888: F, t632: F, t1102: F, t10810: F, t2877: F, t10770: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10878 = 6.399008129061525 * t1063 * t274;
    let t10881 = 2.4210827305188265 * t169 * t2818 * t242;
    let t10883 = t169 * t1113 * t1143;
    let t10886 = t169 * t703 * t2929;
    let t10897 = 0.21223777183119583 * t169 * t699 * t2929;
    let t10903 = t169 * t2888 * t632;
    let t10906 = t169 * t1102 * t1143;
    let t10909 = t169 * t10810 * t242;
    let t10913 = 2.0752137690161367 * t169 * t2877 * t632;
    let t10915 = t169 * t10770 * t242;
    (t10878, t10881, t10883, t10886, t10897, t10903, t10906, t10909, t10913, t10915)
}
