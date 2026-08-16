//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 775/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk775(t10: f64, t127: f64, t1832: f64, t1852: f64, t411: f64, t426: f64, t5596: f64, t7109: f64, t7112: f64, t7115: f64, t7116: f64, t7123: f64, t7128: f64, t7129: f64, t7133: f64, t7137: f64, t7164: f64) -> f64 {
    let t7166 = t7109 + t7112 + t7115 - 29.3808_f64 * t127 * t7116 * t411 + 11.75232_f64 * t127 * t1852 * t1832 - 1.46904_f64 * t127 * t7123 - t7128 - 6.0_f64 * t426 * t10 * t7129 + 3.0_f64 * t426 * t10 * t7133 + 3.0_f64 / 2.0_f64 * t426 * t10 * t7137 - t5596 + t7164;
    t7166
}
