//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 382/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk382(t604: f64, t20: f64, t2454: f64, t649: f64, t1776: f64, t2063: f64, t1775: f64, t2399: f64) -> (f64, f64, f64, f64, f64) {
    let t659 = 0.0_f64 < t604;
    let t2455 = t2454 * t20;
    let t2456 = t649 * t2455;
    let t2459 = t1776 * t2063;
    let t2460 = t1775 * t2459;
    let t2464 = piecewise3(t659, t2399, -t2399);
    (t2455, t2456, t2459, t2460, t2464)
}
