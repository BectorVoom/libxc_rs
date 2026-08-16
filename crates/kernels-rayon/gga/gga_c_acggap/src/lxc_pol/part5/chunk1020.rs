//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1020/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1020(t1165: f64, t12991: f64, t3044: f64, t530: f64, t5082: f64, t935: f64, t3409: f64, t5192: f64, t15758: f64, t3451: f64, t540: f64, t3621: f64, t4571: f64) -> (f64, f64, f64, f64, f64) {
    let t17314 = t12991 * t1165 * t530 * t3044;
    let t17316 = t935 * t5082;
    let t17318 = t3409 * t5192;
    let t17327 = t3451 * t1165 * t540 * t15758;
    let t17353 = t3621 * t4571;
    (t17314, t17316, t17318, t17327, t17353)
}
