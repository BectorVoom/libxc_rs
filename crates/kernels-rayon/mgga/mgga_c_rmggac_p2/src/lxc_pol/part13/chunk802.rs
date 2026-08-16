//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 802/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk802(t2227: f64, t4616: f64, t35861: f64, t36156: f64, t36173: f64, t36200: f64, t36204: f64, t36034: f64, t275: f64, t8202: f64, t35496: f64, t8048: f64, t942: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37423 = t4616 * t2227;
    let t37439 = 0.13659505348792789029e1_f64 * t35861;
    let t37536 = 0.60578599471980134109e-2_f64 * t36156;
    let t37544 = 0.13798458768617697214e-2_f64 * t36173;
    let t37558 = 0.45531684495975963429e0_f64 * t36200;
    let t37560 = 0.10371105912972302781e0_f64 * t36204;
    let t37584 = 0.31113317738916908344e0_f64 * t36034;
    let t37720 = t275 * t8202;
    let t37731 = 0.12649025447177706166e-6_f64 * t35496;
    let t37764 = t942 * t8048;
    (t37423, t37439, t37536, t37544, t37558, t37560, t37584, t37720, t37731, t37764)
}
