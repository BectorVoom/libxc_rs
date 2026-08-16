//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 738/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk738(t1145: f64, t454: f64, t1: f64, t3107: f64, t1781: f64, t321: f64, t429: f64, t457: f64, t4463: f64, t8193: f64, t438: f64, t8196: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8995 = t1145 * t1145;
    let t8996 = 1.0_f64 / t8995;
    let t8997 = t454 * t8996;
    let t9073 = t3107 * t1;
    let t9091 = t321 * t1781 * t429;
    let t9093 = 0.32196894406625029092e-1_f64 * t457 * t9091;
    let t9102 = t4463 * t8193;
    let t9104 = t8196 * t438;
    (t8995, t8996, t8997, t9073, t9091, t9093, t9102, t9104)
}
