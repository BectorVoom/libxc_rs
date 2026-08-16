//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1756/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1756(t3704: f64, t5293: f64, t1802: f64, t3147: f64, t3597: f64, t3594: f64, t1244: f64, t3172: f64, t5286: f64, t1247: f64, t3707: f64, t5292: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17509 = 0.15244095330869239812e-2_f64 * t5293 * t3704;
    let t17523 = t1802 * t3147;
    let t17524 = t3597 * t17523;
    let t17525 = t3594 * t17524;
    let t17528 = t1244 * t17523;
    let t17529 = t3594 * t17528;
    let t17544 = t3172 * t5286;
    let t17546 = 0.28582678745379824648e-3_f64 * t1247 * t17544;
    let t17547 = t3707 * t5292;
    (t17509, t17524, t17525, t17528, t17529, t17544, t17546, t17547)
}
