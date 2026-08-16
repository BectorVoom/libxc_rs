//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 941/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk941(t1245: f64, t187: f64, t22: f64, t1318: f64, t3769: f64, t3899: f64, t1446: f64, t3735: f64, t1479: f64, t3762: f64, t571: f64, t1484: f64, t155: f64) -> (f64, f64, f64, f64, f64) {
    let t10567 = t22 / t187 / t1245;
    let t10574 = t1318 * t3899 * t3769;
    let t10598 = t1446 * t3735;
    let t10603 = t571 * t3762 * t1479;
    let t10605 = t155 * t1484;
    (t10567, t10574, t10598, t10603, t10605)
}
