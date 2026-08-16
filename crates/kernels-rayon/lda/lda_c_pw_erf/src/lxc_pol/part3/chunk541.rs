//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 541/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk541(t168: f64, t2782: f64, t286: f64, t142: f64, t1568: f64, t1724: f64, t454: f64, t1549: f64, t1734: f64, t1704: f64, t1554: f64, t455: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2783 = t168 * t2782;
    let t2785 = 0.19513566535229734_f64 * t2783 * t286;
    let t2786 = t142 * t1568;
    let t2790 = t454 * t1724;
    let t2791 = t2790 * t142;
    let t2793 = t1549 * t1734;
    let t2798 = t142 * t1704;
    let t2799 = t1554 * t2798;
    let t2801 = t455 * t2786;
    (t2783, t2785, t2786, t2790, t2791, t2793, t2798, t2799, t2801)
}
