//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 611/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk611(t2966: f64, t743: f64, t34: f64, t950: f64, t1081: f64, t1772: f64, t1051: f64, t1765: f64, t1055: f64, t1798: f64, t75: f64, t402: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4367 = t2966 * t743;
    let t4370 = t950 * t34;
    let t4387 = t1772 * t1081;
    let t4389 = t1765 * t1051;
    let t4391 = t1765 * t1055;
    let t4393 = t1798 * t75;
    let t4394 = t4393 * t402;
    (t4367, t4370, t4387, t4389, t4391, t4393, t4394)
}
