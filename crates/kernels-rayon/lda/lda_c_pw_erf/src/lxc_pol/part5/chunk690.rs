//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 690/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk690(t100: f64, t411: f64, t142: f64, t1859: f64, t1554: f64, t169: f64, t2357: f64, t301: f64, t717: f64, t2363: f64, t462: f64, t159: f64, t285: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6126 = t411 * t100;
    let t6129 = t142 * t1859;
    let t6130 = t1554 * t6129;
    let t6136 = t169 * t717 * t2357 * t301;
    let t6138 = t462 * t2363;
    let t6140 = t6138 * t159 * t285;
    (t6126, t6129, t6130, t6136, t6138, t6140)
}
