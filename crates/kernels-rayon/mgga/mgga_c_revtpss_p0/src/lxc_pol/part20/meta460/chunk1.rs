//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1752/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1752(t3869: f64, t39430: f64, t9572: f64, t9860: f64, t39742: f64, t39440: f64, t9866: f64, t9863: f64, t40072: f64, t47107: f64, t47109: f64, t47111: f64, t47114: f64, t47116: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47118 = 0.38527786510141256862e1_f64 * t3869 * t39430;
    let t47119 = t9860 * t9572;
    let t47120 = 0.1301229756036208781e0_f64 * t47119;
    let t47122 = 0.1301229756036208781e0_f64 * t3869 * t39742;
    let t47124 = 0.67471172535210825684e-1_f64 * t3869 * t39440;
    let t47125 = t9860 * t9866;
    let t47126 = 0.19263893255070628431e1_f64 * t47125;
    let t47127 = t9860 * t9863;
    let t47128 = 0.65061487801810439052e-1_f64 * t47127;
    let t47129 = -t40072 - t47107 - t47109 - t47111 + t47114 + t47116 - t47118 - t47120 + t47122 + t47124 + t47126 + t47128;
    (t47118, t47120, t47122, t47124, t47126, t47128, t47129)
}
