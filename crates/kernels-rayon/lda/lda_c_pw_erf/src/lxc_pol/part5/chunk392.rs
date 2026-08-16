//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 392/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk392(t43: f64, t1775: f64, t40: f64, t339: f64, t749: f64, t344: f64, t739: f64, t939: f64, t34: f64, t47: f64, t348: f64, t462: f64, t743: f64, t950: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t1776 = t40 * t1775;
    let t1777 = t339 * t749;
    let t1778 = 4.0_f64 * t1777;
    let t1779 = t344 * t749;
    let t1780 = 4.0_f64 * t1779;
    let t1781 = t939 * t739;
    let t1784 = t47 * t34;
    let t1788 = piecewise3(t44, 0.0_f64, 4.0_f64 / 9.0_f64 * t1781 * t348 + 8.0_f64 / 3.0_f64 * t1784 * t462);
    let t1789 = t950 * t743;
    (t1776, t1777, t1778, t1779, t1780, t1781, t1788, t1789)
}
