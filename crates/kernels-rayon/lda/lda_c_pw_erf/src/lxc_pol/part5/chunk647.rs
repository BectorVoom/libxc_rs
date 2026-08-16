//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 647/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk647(t2104: f64, t2137: f64, t1284: f64, t1298: f64, t2127: f64, t2134: f64, t511: f64, t2114: f64, t1958: f64, t202: f64, t184: f64, t172: f64, t1980: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5190 = 16.0_f64 / 45.0_f64 * t2104 * t2137;
    let t5192 = 16.0_f64 / 45.0_f64 * t1284 * t2137;
    let t5194 = 16.0_f64 / 45.0_f64 * t1298 * t2127;
    let t5198 = 8.0_f64 / 45.0_f64 * t511 * t2134;
    let t5200 = 16.0_f64 / 45.0_f64 * t2114 * t2127;
    let t5210 = t202 * t1958;
    let t5211 = t5210 * t184;
    let t5214 = t172 * t1980;
    (t5190, t5192, t5194, t5198, t5200, t5210, t5211, t5214)
}
