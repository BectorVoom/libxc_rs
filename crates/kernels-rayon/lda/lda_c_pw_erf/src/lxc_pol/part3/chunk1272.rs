//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1272/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1272(t1: f64, t3921: f64, t5470: f64, t12461: f64, t12463: f64, t12465: f64, t12474: f64, t12480: f64, t12482: f64, t12485: f64, t12488: f64, t12491: f64, t12495: f64, t12497: f64, t12499: f64) -> f64 {
    let t15015 = t5470 * t1 * t3921;
    let t15017 = -t12461 - t12463 + t12465 - t12474 - t12480 - t12482 - t12485 - t12488 - t12491 - t12495 + 0.001515438175925926_f64 * t15015 - t12497 + t12499;
    t15017
}
