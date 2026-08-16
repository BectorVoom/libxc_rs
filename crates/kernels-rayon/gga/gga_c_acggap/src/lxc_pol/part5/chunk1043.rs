//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1043/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1043(t3431: f64, t5277: f64, t1165: f64, t12349: f64, t1532: f64, t3456: f64, t1163: f64, t16548: f64, t540: f64, t12738: f64, t5147: f64, t1008: f64, t5118: f64) -> (f64, f64, f64, f64, f64) {
    let t18037 = t3431 * t5277;
    let t18041 = t3456 * t1165 * t1532 * t12349;
    let t18045 = t1163 * t1165 * t540 * t16548;
    let t18047 = t12738 * t5147;
    let t18062 = t1008 * t5118;
    (t18037, t18041, t18045, t18047, t18062)
}
