//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 455/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk455(t1: f64, t1750: f64, t305: f64, t152: f64, t6: f64, t1124: f64, t279: f64, t19: f64, t726: f64, t729: f64, t748: f64, t75: f64) -> (f64, f64, f64, f64, f64) {
    let t1752 = t305 * t1750 * t1;
    let t1753 = t152 * t6;
    let t1755 = t1753 * t1124 * t279;
    let t1759 = t726 * t729 * t19;
    let t1765 = t748 * t75;
    (t1752, t1753, t1755, t1759, t1765)
}
