//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1192/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1192(t21616: f64, t558: f64, t7456: f64, t1308: f64, t352: f64, t571: f64, t10167: f64, t1558: f64, t1563: f64, t17673: f64, t19994: f64, t19997: f64, t20007: f64, t20019: f64, t20027: f64, t2268: f64, t2274: f64, t348: f64, t5812: f64, t5823: f64, t5992: f64, t6005: f64, t6164: f64, t6174: f64, t659: f64, t661: f64, t7354: f64, t7360: f64, t7365: f64, t7370: f64, t753: f64, t754: f64, t92: f64, t93: f64, t943: f64) -> (f64, f64, f64, f64) {
    let t21617 = 16.0_f64 / 45.0_f64 * t21616;
    let t21618 = t7456 * t558;
    let t21622 = 4.0_f64 / 45.0_f64 * t571 * t1308 * t21618 * t352;
    let t21624 = 32.0_f64 / 1215.0_f64 * t10167;
    let t21657 = -40.0_f64 / 81.0_f64 * t1558 * t7354 * t348 + 80.0_f64 / 9.0_f64 * t6164 * t943 + 40.0_f64 / 9.0_f64 * t753 * t19994 + 40.0_f64 / 3.0_f64 * t5812 * t19997 + 20.0_f64 / 3.0_f64 * t2268 * t5992 + 20.0_f64 / 9.0_f64 * t92 * t7360 * t348 + 4.0_f64 / 3.0_f64 * t659 * t20007 - 40.0_f64 / 81.0_f64 * t1563 * t7365 * t352 - 80.0_f64 / 9.0_f64 * t6174 * t943 + 40.0_f64 / 9.0_f64 * t754 * t17673 - 40.0_f64 / 3.0_f64 * t5823 * t20019 + 20.0_f64 / 3.0_f64 * t2274 * t6005 + 20.0_f64 / 9.0_f64 * t93 * t7370 * t352 + 4.0_f64 / 3.0_f64 * t661 * t20027;
    (t21617, t21622, t21624, t21657)
}
