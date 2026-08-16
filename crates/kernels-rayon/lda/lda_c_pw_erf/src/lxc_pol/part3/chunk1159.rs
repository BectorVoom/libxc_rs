//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1159/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1159(t11: f64, t13368: f64, t557: f64, t12254: f64, t3633: f64, t13344: f64, t1349: f64, t1953: f64, t13294: f64, t190: f64, t4981: f64, t9821: f64) -> (f64, f64, f64, f64, f64) {
    let t13571 = t11 * t557 * t13368;
    let t13574 = t11 * t3633 * t12254;
    let t13577 = t1953 * t1349 * t13344;
    let t13580 = t1953 * t557 * t13294;
    let t13583 = t190 * t9821 * t4981;
    (t13571, t13574, t13577, t13580, t13583)
}
