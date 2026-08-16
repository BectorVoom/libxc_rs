//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 937/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk937(t3088: f64, t3089: f64, t955: f64, t13326: f64, t183: f64, t14402: f64, t453: f64, t1035: f64, t1240: f64, t3044: f64, t381: f64, t3828: f64, t879: f64) -> (f64, f64, f64, f64, f64) {
    let t14551 = t3088 * t3089 * t955;
    let t14554 = 0.65854491829355115987e0_f64 * t13326 * t183;
    let t14556 = 0.26341796731742046395e1_f64 * t14402 * t453;
    let t14564 = t1035 * t1240 * t3044;
    let t14570 = t381 * t3828 * t879;
    (t14551, t14554, t14556, t14564, t14570)
}
