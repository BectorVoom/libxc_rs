//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1366/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1366(t198: f64, t205: f64, t6353: f64, t1692: f64, t1989: f64, t6354: f64, t18728: f64, t69868: f64, t18807: f64, t19672: f64, t19685: f64, t20417: f64, t21263: f64, t21266: f64, t21353: f64, t21359: f64, t2439: f64, t4578: f64, t5849: f64, t5853: f64, t62610: f64, t62829: f64, t69789: f64, t69811: f64, t69828: f64, t69842: f64, t70227: f64, t70261: f64) -> (f64, f64, f64, f64) {
    let t72279 = t198 * t205 * t6353;
    let t72298 = 2.0_f64 * t1692 * t6354 * t1989;
    let t72310 = 6.0_f64 * t18728 * t69868;
    let t72317 = 6.0_f64 * t72279 * t19672 + 3.0_f64 * t2439 * t5849 * t21266 - t1692 * t5853 * t70261 / 2.0_f64 - 3.0_f64 * t62610 * t21263 + 3.0_f64 * t2439 * t6354 * t19685 - t1692 * t5853 * t70227 / 2.0_f64 + t72298 + t1692 * t62829 * t21353 + t1692 * t5849 * t4578 / 2.0_f64 - 3.0_f64 * t18728 * t69789 - t1692 * t18807 * t21359 / 2.0_f64 + t72310 + 3.0_f64 * t20417 * t69842 - 3.0_f64 * t18728 * t69828 - 3.0_f64 * t18728 * t69811;
    (t72279, t72298, t72310, t72317)
}
