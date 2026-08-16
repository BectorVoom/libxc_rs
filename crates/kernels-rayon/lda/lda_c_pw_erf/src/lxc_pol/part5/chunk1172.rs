//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1172/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1172(t17107: f64, t17109: f64, t17112: f64, t17114: f64, t2134: f64, t2407: f64, t17117: f64, t12475: f64, t6442: f64, t6762: f64, t2325: f64, t806: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21387 = 8.0_f64 / 15.0_f64 * t17107;
    let t21388 = 8.0_f64 / 15.0_f64 * t17109;
    let t21389 = 8.0_f64 / 15.0_f64 * t17112;
    let t21390 = 16.0_f64 / 15.0_f64 * t17114;
    let t21391 = t2407 * t2134;
    let t21392 = 8.0_f64 / 15.0_f64 * t21391;
    let t21393 = 4.0_f64 / 45.0_f64 * t17117;
    let t21396 = 64.0_f64 / 15.0_f64 * t12475 * t6762 * t6442;
    let t21397 = t2325 * t806;
    (t21387, t21388, t21389, t21390, t21392, t21393, t21396, t21397)
}
