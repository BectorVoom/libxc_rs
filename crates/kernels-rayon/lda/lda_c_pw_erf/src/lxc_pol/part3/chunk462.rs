//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 462/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk462(t1660: f64, t415: f64, t760: f64, t325: f64, t102: f64, t411: f64, t763: f64, t1558: f64, t739: f64, t34: f64, t406: f64, t1563: f64, t743: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1813 = 0.48717083333333333_f64 * t1660;
    let t1814 = t415 * t760;
    let t1815 = t1814 * t325;
    let t1816 = 0.48717083333333333_f64 * t1815;
    let t1819 = 5.84605_f64 * t102 * t763 * t411;
    let t1820 = t1558 * t739;
    let t1823 = t406 * t34;
    let t1826 = t1563 * t743;
    (t1813, t1814, t1816, t1819, t1820, t1823, t1826)
}
