//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1077/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1077(t5236: f64, t5257: f64, t1732: f64, t6895: f64, t167: f64, t168: f64, t16942: f64, t180: f64, t66: f64, t5221: f64, t5261: f64, t16405: f64) -> (f64, f64, f64, f64, f64) {
    let t17056 = t5257 * t5236;
    let t17067 = t6895 * t1732;
    let t17088 = 0.28974367305964659283e0_f64 * t167 * t168 / t66 / t16942 * t180;
    let t17089 = t5221 * t5261;
    let t17095 = t167 * t16405;
    (t17056, t17067, t17088, t17089, t17095)
}
