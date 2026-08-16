//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1097/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1097(t11152: f64, t76: f64, t2244: f64, t2250: f64, t2251: f64, t2252: f64, t2255: f64, t2283: f64, t2284: f64, t2291: f64, t2298: f64, t2304: f64, t39096: f64, t39097: f64, t39103: f64, t39110: f64, t608: f64, t609: f64, t629: f64, t634: f64, t638: f64, t642: f64, t66: f64, t72: f64, t80: f64, t9258: f64, t9263: f64, t9268: f64, t9312: f64, t9313: f64, t9321: f64, t9324: f64, t9330: f64, t9333: f64, t9339: f64) -> f64 {
    let t39114 = 1.0_f64 / t76 / t11152;
    let t39130 = -t2251 * t2283 * t80 / 2.0_f64 - t9263 * t642 - t2252 * t2304 / 2.0_f64 - t608 * t9312 * t80 / 3.0_f64 - t9268 * t642 - t2255 * t2304 - t609 * t9339 / 3.0_f64 + t9313 * t642 / 6.0_f64 + t2284 * t2304 / 4.0_f64 + t629 * t9339 / 6.0_f64 + t66 * t72 * (3640.0_f64 / 81.0_f64 * t39096 * t39097 - 560.0_f64 / 9.0_f64 * t9321 * t2244 * t2250 + 28.0_f64 / 3.0_f64 * t2291 * t39103 + 112.0_f64 / 9.0_f64 * t9324 * t9258 - 4.0_f64 / 3.0_f64 * t634 * t39110 + 3640.0_f64 / 81.0_f64 * t39114 * t39097 + 560.0_f64 / 9.0_f64 * t9330 * t2244 * t2250 + 28.0_f64 / 3.0_f64 * t2298 * t39103 + 112.0_f64 / 9.0_f64 * t9333 * t9258 + 4.0_f64 / 3.0_f64 * t638 * t39110) / 24.0_f64;
    t39130
}
