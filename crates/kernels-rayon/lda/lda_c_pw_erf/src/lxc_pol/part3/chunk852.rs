//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 852/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk852(t4574: f64, t811: f64, t5165: f64, t4722: f64, t784: f64, t5146: f64, t549: f64, t820: f64, t184: f64, t4387: f64, t4389: f64, t4391: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6748 = t4574 * t811;
    let t6752 = t5165 * t811;
    let t6762 = t4722 * t784;
    let t6766 = t5146 * t784;
    let t6850 = t549 * t820;
    let t6851 = t6850 * t184;
    let t7324 = 0.0007324622014701264_f64 * t4387;
    let t7325 = 1.7544670192365612_f64 * t4389;
    let t7326 = 51.94726769812759_f64 * t4391;
    (t6748, t6752, t6762, t6766, t6851, t7324, t7325, t7326)
}
