//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 632/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk632(t2849: f64, t1931: f64, t611: f64, t1621: f64, t838: f64, t197: f64, t521: f64, t1518: f64, t807: f64, t185: f64, t230: f64, t610: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4712 = 12.0_f64 * t2849;
    let t4718 = 8.0_f64 / 3.0_f64 * t1931 * t611;
    let t4719 = t838 * t1621;
    let t4722 = t521 * t197;
    let t4729 = t1518 * t807;
    let t4730 = t185 * t4729;
    let t4733 = 8.0_f64 / 3.0_f64 * t1931 * t230;
    let t4734 = t838 * t610;
    (t4712, t4718, t4719, t4722, t4729, t4730, t4733, t4734)
}
