//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1820/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1820(t9211: f64, t9213: f64, t9215: f64, t9217: f64, t9219: f64, t9221: f64, t9225: f64, t1437: f64, t5389: f64, t5445: f64, t1864: f64, t5398: f64) -> (f64, f64, f64, f64) {
    let t20193 = -t9211 - t9213 - t9215 - t9217 - t9219 - t9221 - t9225;
    let t20201 = t5389 * t1437;
    let t20204 = t1437 * t5445;
    let t20207 = t1864 * t5398;
    (t20193, t20201, t20204, t20207)
}
