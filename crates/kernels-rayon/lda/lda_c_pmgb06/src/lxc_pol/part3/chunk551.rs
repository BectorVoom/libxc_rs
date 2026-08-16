//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 551/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk551(t1594: f64, t454: f64, t2864: f64, t439: f64, t1382: f64, t1447: f64, t1600: f64, t496: f64, t1602: f64, t507: f64, t493: f64, t1481: f64, t529: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2865 = t454 * t1594;
    let t2866 = t2864 * t2865;
    let t2868 = 2.0_f64 / 15.0_f64 * t439 * t2866;
    let t2869 = t1447 * t1382;
    let t2870 = 4.0_f64 / 45.0_f64 * t2869;
    let t2871 = t496 * t1600;
    let t2872 = t507 * t1602;
    let t2873 = t2871 * t2872;
    let t2875 = 2.0_f64 / 15.0_f64 * t493 * t2873;
    let t2876 = t1481 * t529;
    (t2865, t2866, t2868, t2869, t2870, t2871, t2872, t2873, t2875, t2876)
}
