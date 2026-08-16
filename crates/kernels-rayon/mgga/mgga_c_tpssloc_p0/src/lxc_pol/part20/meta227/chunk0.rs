//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1307/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1307(t9256: f64, t95: f64, t101: f64, t102: f64, t2350: f64, t662: f64, t2349: f64, t2354: f64, t103: f64, t100: f64, t2336: f64, t2343: f64, t2346: f64, t657: f64, t660: f64, t92: f64, t9374: f64, t9386: f64, t9390: f64, t96: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9393 = 3.0_f64 * t9256;
    let t9394 = t95 * t9393;
    let t9397 = t102 * t101;
    let t9398 = 1.0_f64 / t9397;
    let t9399 = t2350 * t662;
    let t9400 = t9398 * t9399;
    let t9403 = t2349 * t662;
    let t9404 = t9403 * t2354;
    let t9407 = -t9393;
    let t9408 = t103 * t9407;
    let t9411 = -440.0_f64 / 27.0_f64 * t9374 * t96 + 200.0_f64 / 9.0_f64 * t2336 * t660 - 50.0_f64 / 9.0_f64 * t657 * t2343 - 25.0_f64 / 3.0_f64 * t657 * t2346 - 10.0_f64 / 27.0_f64 * t92 * t9386 + 10.0_f64 / 3.0_f64 * t92 * t9390 + 5.0_f64 / 3.0_f64 * t92 * t9394 - 10.0_f64 / 27.0_f64 * t100 * t9400 + 10.0_f64 / 3.0_f64 * t100 * t9404 + 5.0_f64 / 3.0_f64 * t100 * t9408;
    (t9393, t9398, t9399, t9400, t9404, t9407, t9408, t9411)
}
