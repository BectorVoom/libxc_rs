//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 913/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk913(t108: f64, t1297: f64, t267: f64, t1283: f64, t3973: f64, t668: f64) -> (f64, f64, f64) {
    let t10015 = t1297 * t108 * t267;
    let t10027 = t1283 * t108 * t267;
    let t10030 = t3973 * t668;
    (t10015, t10027, t10030)
}
