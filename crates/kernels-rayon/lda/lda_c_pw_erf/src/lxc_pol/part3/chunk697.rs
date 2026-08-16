//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 697/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk697(t1752: f64, t4299: f64, t1746: f64, t1759: f64, t1742: f64, t19: f64, t729: f64, t734: f64, t2953: f64, t739: f64, t34: f64, t939: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4300 = t1752 * t4299;
    let t4304 = t1759 * t1746;
    let t4307 = t1742 * t729 * t19;
    let t4308 = t4307 * t734;
    let t4352 = t2953 * t739;
    let t4355 = t939 * t34;
    (t4300, t4304, t4307, t4308, t4352, t4355)
}
