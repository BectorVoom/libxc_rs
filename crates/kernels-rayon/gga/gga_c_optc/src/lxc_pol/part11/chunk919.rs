//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 919/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk919(t17133: f64, t17173: f64, t17206: f64, t17243: f64, t106: f64, t11130: f64, t1411: f64, t14472: f64, t17079: f64, t17092: f64, t17096: f64, t335: f64, t3853: f64, t3860: f64, t4990: f64, t5049: f64, t7948: f64, t908: f64) -> (f64, f64) {
    let t17245 = t17133 + t17173 + t17206 + t17243;
    let t17249 = 0.27818116767324025134e1_f64 * t106 * t17079 * t335 - 0.83454350301972075402e1_f64 * t106 * t14472 * t1411 + 0.16690870060394415081e2_f64 * t106 * t11130 * t4990 - 0.83454350301972075402e1_f64 * t106 * t3853 * t5049 - 0.1669087006039441508e2_f64 * t106 * t7948 * t17092 + 0.16690870060394415081e2_f64 * t3860 * t17096 - 0.27818116767324025134e1_f64 * t106 * t908 * t17245;
    (t17245, t17249)
}
