//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1251/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1251(t11561: f64, t11863: f64, t11864: f64, t11618: f64, t11623: f64, t11631: f64, t11634: f64, t11637: f64, t12020: f64, t11858: f64, t11001: f64, t11006: f64, t11014: f64, t11022: f64, t11025: f64, t11627: f64) -> f64 {
    let t41104 = 5.0_f64 / 8.0_f64 * t11561;
    let t41105 = 2.0_f64 * t11863;
    let t41106 = 2.0_f64 * t11864;
    let t41107 = 5.0_f64 / 8.0_f64 * t11618;
    let t41108 = 45.0_f64 / 32.0_f64 * t11623;
    let t41109 = 5.0_f64 / 8.0_f64 * t11631;
    let t41110 = t11634 / 2.0_f64;
    let t41111 = 3.0_f64 / 2.0_f64 * t11637;
    let t41112 = 2.0_f64 * t12020;
    let t41113 = t11858 / 2.0_f64;
    let t41114 = t41104 + t41105 + t41106 + t11001 - t41107 - t11006 + t41108 - t11014 + t11627 - t41109 - t41110 + t41111 + t11022 + t41112 + t11025 + t41113;
    t41114
}
