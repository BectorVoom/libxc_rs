//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1049/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1049(t1077: f64, t5967: f64, t1081: f64, t6055: f64, t1: f64, t397: f64, t6011: f64, t339: f64, t6069: f64, t2357: f64, t39: f64, t1217: f64, t2455: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18973 = t5967 * t1077;
    let t18976 = t6055 * t1081;
    let t18981 = t6011 * t1 * t397;
    let t18998 = t339 * t6069;
    let t19008 = t39 * t2357;
    let t19123 = t2455 * t1217;
    (t18973, t18976, t18981, t18998, t19008, t19123)
}
