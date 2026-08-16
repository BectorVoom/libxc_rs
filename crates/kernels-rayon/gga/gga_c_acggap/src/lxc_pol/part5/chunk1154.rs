//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1154/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1154(t1181: f64, t1532: f64, t1753: f64, t3451: f64, t864: f64, t14050: f64, t5737: f64, t1165: f64, t14187: f64, t301: f64, t3457: f64, t5852: f64) -> (f64, f64, f64) {
    let t20826 = t3451 * t1181 * t1532 * t1753 * t864;
    let t20830 = t14050 * t5737;
    let t20836 = t14187 * t1165 * t5852 * t3457 * t301;
    (t20826, t20830, t20836)
}
