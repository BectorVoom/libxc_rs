//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 851/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk851(t100: f64, t411: f64, t2775: f64, t776: f64, t101: f64, t494: f64, t793: f64, t184: f64, t4489: f64, t504: f64, t4507: f64, t558: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6126 = t411 * t100;
    let t6153 = t776 * t2775;
    let t6154 = t101 * t6153;
    let t6579 = t494 * t793;
    let t6580 = t6579 * t184;
    let t6710 = t4489 * t504;
    let t6728 = t4507 * t558;
    (t6126, t6153, t6154, t6580, t6710, t6728)
}
