//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 823/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk823(t5275: f64, t861: f64, t24: f64, t5318: f64, t1111: f64, t5289: f64, t1146: f64, t5344: f64, t106: f64, t1523: f64, t5351: f64, t8996: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15690 = t5275 * t861;
    let t15693 = t24 * t5318;
    let t15694 = t1111 * t15693;
    let t15696 = t24 * t5289;
    let t15697 = t1111 * t15696;
    let t15706 = t5344 * t1146;
    let t15713 = t106 * t1523;
    let t15722 = t8996 * t5351;
    (t15690, t15693, t15694, t15696, t15697, t15706, t15713, t15722)
}
