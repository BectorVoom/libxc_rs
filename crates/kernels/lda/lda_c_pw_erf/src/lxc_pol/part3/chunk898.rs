//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 898/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk898<F: Float>(t120: F, t133: F, t2869: F, t1870: F, t3357: F, t5639: F, t8939: F, t8942: F, t8945: F, t9024: F, t9021: F, t2775: F, t452: F) -> (F, F, F, F, F, F, F, F) {
    let t9083 = F::cast_from(2.9801938271604937_f64) * t133 * t2869 * t120;
    let t9094 = t1870 * t5639 * t3357;
    let t9096 = t133 * t8939;
    let t9098 = t133 * t8942;
    let t9100 = t133 * t8945;
    let t9104 = t133 * t9024;
    let t9110 = t133 * t9021;
    let t9118 = t452 * t2775;
    (t9083, t9094, t9096, t9098, t9100, t9104, t9110, t9118)
}
