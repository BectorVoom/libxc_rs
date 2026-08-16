//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 866/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk866(t1759: f64, t4295: f64, t1059: f64, t2948: f64, t2979: f64, t402: f64, t75: f64, t390: f64, t40: f64, t3189: f64, t344: f64, t339: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8301 = t1759 * t4295;
    let t8303 = t1059 * t2948;
    let t8306 = t2979 * t75 * t402;
    let t8309 = t40 * t2979 * t390;
    let t8311 = t344 * t3189;
    let t8313 = t339 * t3189;
    (t8301, t8303, t8306, t8309, t8311, t8313)
}
