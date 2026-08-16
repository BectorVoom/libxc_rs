//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1225/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1225(t2146: f64, t6389: f64, t10419: f64, t17979: f64, t17981: f64, t17983: f64, t22084: f64, t22086: f64, t22088: f64, t22093: f64, t22098: f64, t22102: f64, t22107: f64, t22109: f64, t22111: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22113 = 16.0_f64 / 9.0_f64 * t2146 * t6389;
    let t22114 = 32.0_f64 / 405.0_f64 * t10419;
    let t22115 = 8.0_f64 / 15.0_f64 * t17979;
    let t22116 = 8.0_f64 / 15.0_f64 * t17981;
    let t22117 = 8.0_f64 / 15.0_f64 * t17983;
    let t22118 = -t22084 + t22086 + t22088 + t22093 - t22098 + t22102 - t22107 - t22109 + t22111 - t22113 + t22114 - t22115 - t22116 + t22117;
    (t22113, t22114, t22115, t22116, t22117, t22118)
}
