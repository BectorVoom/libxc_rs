//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1117/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1117(t13066: f64, t4581: f64, t4753: f64, t3416: f64, t1318: f64, t3854: f64, t5225: f64, t13042: f64, t13044: f64, t13046: f64, t13049: f64, t13052: f64, t13055: f64, t13057: f64, t13059: f64, t13064: f64) -> (f64, f64, f64, f64, f64) {
    let t13067 = 8.0_f64 / 27.0_f64 * t13066;
    let t13068 = t4753 * t4581;
    let t13069 = 32.0_f64 / 45.0_f64 * t13068;
    let t13070 = t3416 * t4581;
    let t13071 = 32.0_f64 / 45.0_f64 * t13070;
    let t13073 = t1318 * t3854 * t5225;
    let t13074 = 16.0_f64 / 45.0_f64 * t13073;
    let t13075 = -t13042 - t13044 - t13046 + t13049 + t13052 - t13055 - t13057 + t13059 - t13064 - t13067 + t13069 + t13071 + t13074;
    (t13067, t13069, t13071, t13074, t13075)
}
