//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1127/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1127(t3905: f64, t4763: f64, t1446: f64, t4887: f64, t3794: f64, t4882: f64, t1325: f64, t1440: f64, t494: f64, t5127: f64, t529: f64, t13162: f64, t13164: f64, t13166: f64, t13171: f64, t13175: f64, t13177: f64, t13179: f64, t13182: f64, t13187: f64) -> (f64, f64, f64, f64, f64) {
    let t13189 = 4.0_f64 / 5.0_f64 * t4763 * t3905;
    let t13191 = 8.0_f64 / 5.0_f64 * t1446 * t4887;
    let t13193 = 8.0_f64 / 5.0_f64 * t3794 * t4882;
    let t13198 = 4.0_f64 / 5.0_f64 * t1325 * t1440 * t529 * t5127 * t494;
    let t13199 = -t13162 + t13164 - t13166 + t13171 + t13175 - t13177 + t13179 + t13182 - t13187 - t13189 + t13191 - t13193 - t13198;
    (t13189, t13191, t13193, t13198, t13199)
}
