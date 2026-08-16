//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 953/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk953(t17485: f64, t17499: f64, t1085: f64, t1094: f64, t1102: f64, t4300: f64, t5110: f64, t4299: f64, t1492: f64, t15562: f64, t17449: f64, t3058: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17500 = t17485 + t17499;
    let t17502 = t1085 * t17500 * t1094;
    let t17504 = 0.58482233974552040708e0_f64 * t1102 * t17502;
    let t17515 = t4300 * t5110;
    let t17516 = t4299 * t17515;
    let t17527 = 0.17544670192365612213e1_f64 * t15562 * t1492;
    let t17529 = t3058 * t17449 * t1094;
    (t17500, t17502, t17504, t17515, t17516, t17527, t17529)
}
