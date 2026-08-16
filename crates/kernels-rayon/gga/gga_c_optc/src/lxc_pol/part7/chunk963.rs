//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 963/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk963(t9056: f64, t9112: f64, t9164: f64, t9215: f64, t106: f64, t1147: f64, t1182: f64, t3164: f64, t3171: f64, t3264: f64, t4410: f64, t470: f64, t8980: f64, t8984: f64, t8988: f64, t8997: f64, t8998: f64, t9003: f64) -> (f64, f64) {
    let t9217 = t9056 + t9112 + t9164 + t9215;
    let t9221 = 0.27818116767324025134e1_f64 * t106 * t8980 * t470 - 0.83454350301972075402e1_f64 * t106 * t8984 * t1182 + 0.16690870060394415081e2_f64 * t106 * t8988 * t3171 - 0.83454350301972075402e1_f64 * t106 * t3164 * t3264 - 0.1669087006039441508e2_f64 * t106 * t8997 * t8998 + 0.16690870060394415081e2_f64 * t4410 * t9003 - 0.27818116767324025134e1_f64 * t106 * t1147 * t9217;
    (t9217, t9221)
}
