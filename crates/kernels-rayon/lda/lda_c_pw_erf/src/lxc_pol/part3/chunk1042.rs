//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1042/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1042(t10474: f64, t2027: f64, t3794: f64, t4826: f64, t12176: f64, t12178: f64, t12181: f64, t12184: f64, t12186: f64, t12188: f64, t12190: f64, t12194: f64, t12197: f64, t12199: f64, t12203: f64) -> (f64, f64, f64) {
    let t12205 = 8.0_f64 / 15.0_f64 * t10474 * t2027;
    let t12207 = 8.0_f64 / 15.0_f64 * t3794 * t4826;
    let t12208 = -t12176 - t12178 - t12181 - t12184 - t12186 - t12188 - t12190 - t12194 + t12197 - t12199 - t12203 + t12205 + t12207;
    (t12205, t12207, t12208)
}
