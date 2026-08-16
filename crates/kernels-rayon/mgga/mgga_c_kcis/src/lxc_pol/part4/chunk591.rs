//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 591/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk591(t3106: f64, t323: f64, t325: f64, t2394: f64, t41: f64, t335: f64, t333: f64, t1057: f64, t733: f64, t1056: f64, t2829: f64, t2845: f64, t345: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3109 = 0.21133333333333333333e-2_f64 * t323 * t3106 * t325;
    let t3110 = t2394 * t41;
    let t3111 = t3110 * t335;
    let t3113 = 0.16804375e-4_f64 * t333 * t3111;
    let t3114 = t733 * t1057;
    let t3116 = t1056 * t2829;
    let t3119 = t345 * t2845;
    (t3109, t3110, t3111, t3113, t3114, t3116, t3119)
}
