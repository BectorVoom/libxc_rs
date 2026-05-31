//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1192/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1192<F: Float>(t21616: F, t558: F, t7456: F, t1308: F, t352: F, t571: F, t10167: F, t1558: F, t1563: F, t17673: F, t19994: F, t19997: F, t20007: F, t20019: F, t20027: F, t2268: F, t2274: F, t348: F, t5812: F, t5823: F, t5992: F, t6005: F, t6164: F, t6174: F, t659: F, t661: F, t7354: F, t7360: F, t7365: F, t7370: F, t753: F, t754: F, t92: F, t93: F, t943: F) -> (F, F, F, F) {
    let t21617 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t21616;
    let t21618 = t7456 * t558;
    let t21622 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t571 * t1308 * t21618 * t352;
    let t21624 = F::cast_from(32.0_f64) / F::cast_from(1215.0_f64) * t10167;
    let t21657 = -F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t1558 * t7354 * t348 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t6164 * t943 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t753 * t19994 + F::cast_from(40.0_f64) / F::cast_from(3.0_f64) * t5812 * t19997 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t2268 * t5992 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t92 * t7360 * t348 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t659 * t20007 - F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t1563 * t7365 * t352 - F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t6174 * t943 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t754 * t17673 - F::cast_from(40.0_f64) / F::cast_from(3.0_f64) * t5823 * t20019 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t2274 * t6005 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t93 * t7370 * t352 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t661 * t20027;
    (t21617, t21622, t21624, t21657)
}
