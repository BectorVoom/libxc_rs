//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 778/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk778(t9384: f64, t9385: f64, t2341: f64, t659: f64, t2248: f64, t9256: f64, t95: f64, t101: f64, t102: f64, t2350: f64, t662: f64, t2349: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9386 = t9384 * t9385;
    let t9389 = t2341 * t659;
    let t9390 = t9389 * t2248;
    let t9393 = 3.0_f64 * t9256;
    let t9394 = t95 * t9393;
    let t9397 = t102 * t101;
    let t9398 = 1.0_f64 / t9397;
    let t9399 = t2350 * t662;
    let t9400 = t9398 * t9399;
    let t9403 = t2349 * t662;
    (t9386, t9389, t9390, t9393, t9394, t9398, t9400, t9403)
}
