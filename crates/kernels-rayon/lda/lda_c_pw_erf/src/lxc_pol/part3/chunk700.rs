//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 700/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk700(t4383: f64, t87: f64, t40: f64, t2705: f64, t1081: f64, t1772: f64, t1051: f64, t1765: f64, t1055: f64, t1798: f64, t75: f64, t402: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4384 = t4383 * t87;
    let t4385 = t40 * t4384;
    let t4386 = 0.021687161765563047_f64 * t2705;
    let t4387 = t1772 * t1081;
    let t4388 = 0.0002441540671567088_f64 * t4387;
    let t4389 = t1765 * t1051;
    let t4390 = 0.5848223397455204_f64 * t4389;
    let t4391 = t1765 * t1055;
    let t4392 = 17.315755899375862_f64 * t4391;
    let t4393 = t1798 * t75;
    let t4394 = t4393 * t402;
    (t4384, t4385, t4386, t4387, t4388, t4389, t4390, t4391, t4392, t4393, t4394)
}
