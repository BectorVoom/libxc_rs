//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 589/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk589<F: Float>(t2735: F, t379: F, t386: F, t400: F, t1051: F, t1059: F, t75: F, t960: F, t402: F, t155: F, t903: F, t174: F, t908: F, t318: F, t473: F, t335: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2737 = t379 * t2735 * t386;
    let t2738 = t400 * t2737;
    let t2739 = 0.5848223397455204 * t2738;
    let t2740 = t1059 * t1051;
    let t2742 = t960 * t75;
    let t2743 = t2742 * t402;
    let t2745 = t155 * t903;
    let t2747 = t174 * t2745 * t908;
    let t2748 = 0.8591714644109227 * t2747;
    let t2749 = t473 * t318;
    let t2751 = t174 * t2749 * t335;
    (t2737, t2738, t2739, t2740, t2742, t2743, t2745, t2747, t2748, t2749, t2751)
}
