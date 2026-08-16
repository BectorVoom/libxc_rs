//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1118/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1118(t1318: f64, t4794: f64, t5230: f64, t219: f64, t4900: f64, t4759: f64, t4753: f64, t5406: f64, t3416: f64, t3604: f64, t811: f64, t2017: f64, t2967: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13078 = t1318 * t4794 * t5230;
    let t13079 = 16.0_f64 / 27.0_f64 * t13078;
    let t13080 = t4900 * t219;
    let t13082 = t1318 * t13080 * t4759;
    let t13083 = 8.0_f64 / 9.0_f64 * t13082;
    let t13085 = 16.0_f64 / 15.0_f64 * t4753 * t5406;
    let t13087 = 16.0_f64 / 15.0_f64 * t3416 * t5406;
    let t13088 = t811 * t3604;
    let t13092 = 16.0_f64 / 9.0_f64 * t1318 * t2017 * t13088 * t2967;
    (t13079, t13080, t13083, t13085, t13087, t13092)
}
