//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1204/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1204(t40: f64, t52: f64, t2244: f64, t2250: f64, t2291: f64, t39097: f64, t39103: f64, t39110: f64, t634: f64, t75: f64, t767: f64, t9258: f64, t9499: f64, t2298: f64, t638: f64, t771: f64, t78: f64, t9508: f64, zeta_threshold: f64) -> (f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t40833 = piecewise3(t146, 0.0_f64, -56.0_f64 / 81.0_f64 * t2291 * t39097 + 16.0_f64 / 9.0_f64 * t634 * t2244 * t2250 - 2.0_f64 / 3.0_f64 * t75 * t39103 - 8.0_f64 / 9.0_f64 * t9499 * t9258 + 2.0_f64 / 3.0_f64 * t767 * t39110);
    let t40846 = piecewise3(t150, 0.0_f64, -56.0_f64 / 81.0_f64 * t2298 * t39097 - 16.0_f64 / 9.0_f64 * t638 * t2244 * t2250 - 2.0_f64 / 3.0_f64 * t78 * t39103 - 8.0_f64 / 9.0_f64 * t9508 * t9258 - 2.0_f64 / 3.0_f64 * t771 * t39110);
    (t40833, t40846)
}
