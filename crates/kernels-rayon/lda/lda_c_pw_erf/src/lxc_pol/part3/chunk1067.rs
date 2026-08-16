//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1067/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1067(t12498: f64, t184: f64, t202: f64, t4701: f64, t551: f64, t9540: f64, t9590: f64, t12474: f64, t12480: f64, t12482: f64, t12485: f64, t12488: f64, t12491: f64, t12495: f64, t12497: f64) -> (f64, f64, f64, f64, f64) {
    let t12499 = 8.0_f64 / 15.0_f64 * t12498;
    let t12501 = t202 * t4701 * t184;
    let t12503 = 4.0_f64 / 5.0_f64 * t12501 * t551;
    let t12504 = 16.0_f64 / 45.0_f64 * t9540;
    let t12505 = 16.0_f64 / 45.0_f64 * t9590;
    let t12506 = -t12474 - t12480 - t12482 - t12485 - t12488 - t12491 - t12495 - t12497 + t12499 + t12503 + t12504 + t12505;
    (t12499, t12503, t12504, t12505, t12506)
}
