//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 852/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk852(t756: f64, t770: f64, t2765: f64, t2642: f64, t454: f64, t142: f64, t1809: f64, t2610: f64, t5504: f64, t5519: f64, t767: f64, t1820: f64, t1826: f64, t2329: f64, t2337: f64, t3234: f64, t3243: f64, t406: f64, t408: f64, t7354: f64, t7360: f64, t7365: f64, t7370: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7880 = t770 * t756;
    let t7881 = t2765 * t7880;
    let t7886 = t454 * t2642;
    let t7887 = t7886 * t142;
    let t7889 = t1809 * t2610;
    let t7893 = 1.9486833333333333_f64 * t5504;
    let t7896 = 0.9743416666666667_f64 * t5519;
    let t7897 = t767 * t2610;
    let t7913 = 4.0_f64 / 27.0_f64 * t3234 * t7354 - t1820 * t2329 / 3.0_f64 + t406 * t7360 / 3.0_f64 + 4.0_f64 / 27.0_f64 * t3243 * t7365 - t1826 * t2337 / 3.0_f64 + t408 * t7370 / 3.0_f64;
    (t7880, t7881, t7886, t7887, t7889, t7893, t7896, t7897, t7913)
}
