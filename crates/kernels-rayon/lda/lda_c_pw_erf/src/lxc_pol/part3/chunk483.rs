//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 483/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk483(t1349: f64, t1944: f64, t11: f64, t1333: f64, t743: f64, t352: f64) -> (f64, f64, f64, f64) {
    let t1945 = t1349 * t1944;
    let t1946 = t11 * t1945;
    let t1948 = t1333 * t743;
    let t1949 = t1948 * t352;
    (t1945, t1946, t1948, t1949)
}
