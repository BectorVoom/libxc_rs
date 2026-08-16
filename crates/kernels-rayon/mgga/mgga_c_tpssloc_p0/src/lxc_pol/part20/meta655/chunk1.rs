//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2423/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2423(t10623: f64, t4498: f64, t4493: f64, t10629: f64, t14259: f64, t4471: f64, t959: f64, t14260: f64, t2940: f64, t13663: f64, t13718: f64, t49082: f64, t49084: f64, t49086: f64, t49088: f64, t49090: f64, t49092: f64, t49095: f64, t49228: f64, t49244: f64, t49535: f64, t49538: f64, t49540: f64, t49544: f64, t49548: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49550 = 0.51947577317044391277e2_f64 * t10623 * t4498;
    let t49552 = 0.17544670867903938621e1_f64 * t10623 * t4493;
    let t49556 = 0.30762056574649219973e4_f64 * t959 * t10629 * t4471 * t14259;
    let t49558 = 0.30762056574649219973e4_f64 * t2940 * t14260;
    let t49560 = 0.70178683471615754484e1_f64 * t2940 * t13663;
    let t49562 = 0.17544670867903938621e1_f64 * t2940 * t13718;
    let t49563 = -t49082 + t49084 - t49086 + t49088 - t49090 + t49092 - t49095 + t49535 + t49538 - t49540 - t49544 + t49548 - t49550 + t49228 - t49552 - t49556 - t49558 + t49560 - t49562 + t49244;
    (t49550, t49552, t49556, t49558, t49560, t49562, t49563)
}
