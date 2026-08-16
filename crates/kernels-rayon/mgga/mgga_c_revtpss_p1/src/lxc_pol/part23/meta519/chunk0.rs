//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2028/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2028(t17633: f64, t6638: f64, t3626: f64, t12884: f64, t247: f64, t6421: f64, t1261: f64, t20302: f64, t5312: f64, t20298: f64, t1785: f64, t5390: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21227 = t17633 * t6638;
    let t21228 = t3626 * t21227;
    let t21233 = t247 * t12884 * t6421;
    let t21234 = t1261 * t21233;
    let t21236 = t5312 * t20302;
    let t21239 = t5312 * t20298;
    let t21242 = t1785 * t5390;
    (t21227, t21228, t21233, t21234, t21236, t21239, t21242)
}
