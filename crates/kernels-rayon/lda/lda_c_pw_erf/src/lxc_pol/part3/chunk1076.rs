//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1076/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1076(t12600: f64, t2176: f64, t519: f64, t542: f64, t1460: f64, t348: f64, t5255: f64, t10474: f64, t2183: f64, t1325: f64, t494: f64, t523: f64) -> (f64, f64, f64, f64) {
    let t12604 = 8.0_f64 / 15.0_f64 * t519 * t2176 * t12600 * t542;
    let t12608 = 8.0_f64 / 9.0_f64 * t519 * t5255 * t1460 * t348;
    let t12610 = 4.0_f64 / 5.0_f64 * t10474 * t2183;
    let t12614 = 16.0_f64 / 15.0_f64 * t1325 * t2176 * t523 * t494;
    (t12604, t12608, t12610, t12614)
}
