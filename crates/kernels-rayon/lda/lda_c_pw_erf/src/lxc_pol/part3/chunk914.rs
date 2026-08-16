//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 914/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk914(t3384: f64, t511: f64, t1298: f64, t3387: f64, t1386: f64, t3455: f64, t1472: f64, t3763: f64, t3416: f64, t3900: f64, t1401: f64, t1475: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9621 = t511 * t3384;
    let t9627 = t1298 * t3387;
    let t9629 = t3455 * t1386;
    let t9645 = t1472 * t3763;
    let t9647 = t3416 * t3900;
    let t9678 = t1475 * t1401;
    (t9621, t9627, t9629, t9645, t9647, t9678)
}
