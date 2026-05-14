//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1038/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1038<F: Float>(t10167: F, t1558: F, t1563: F, t17673: F, t19994: F, t19997: F, t20007: F, t20019: F, t20027: F, t2268: F, t2274: F, t348: F, t352: F, t5812: F, t5823: F, t5992: F, t6005: F, t6164: F, t6174: F, t659: F, t661: F, t7354: F, t7360: F, t7365: F, t7370: F, t753: F, t754: F, t92: F, t93: F, t943: F) -> (F, F) {
    let t21624 = 32.0 / 1215.0 * t10167;
    let t21657 = -40.0 / 81.0 * t1558 * t7354 * t348 + 80.0 / 9.0 * t6164 * t943 + 40.0 / 9.0 * t753 * t19994 + 40.0 / 3.0 * t5812 * t19997 + 20.0 / 3.0 * t2268 * t5992 + 20.0 / 9.0 * t92 * t7360 * t348 + 4.0 / 3.0 * t659 * t20007 - 40.0 / 81.0 * t1563 * t7365 * t352 - 80.0 / 9.0 * t6174 * t943 + 40.0 / 9.0 * t754 * t17673 - 40.0 / 3.0 * t5823 * t20019 + 20.0 / 3.0 * t2274 * t6005 + 20.0 / 9.0 * t93 * t7370 * t352 + 4.0 / 3.0 * t661 * t20027;
    (t21624, t21657)
}
