//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 782/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk782(t34806: f64, t34921: f64, t35238: f64, t25809: f64, t698: f64, t2227: f64, t4616: f64, t35861: f64, t36156: f64, t36173: f64, t36200: f64, t36204: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37228 = 0.31113317738916908344e0_f64 * t34806;
    let t37266 = 0.1299607316140891005e-4_f64 * t34921;
    let t37375 = 0.91462949374725084936e-3_f64 * t35238;
    let t37419 = t25809 * t698;
    let t37423 = t4616 * t2227;
    let t37439 = 0.13659505348792789029e1_f64 * t35861;
    let t37536 = 0.60578599471980134109e-2_f64 * t36156;
    let t37544 = 0.13798458768617697214e-2_f64 * t36173;
    let t37558 = 0.45531684495975963429e0_f64 * t36200;
    let t37560 = 0.10371105912972302781e0_f64 * t36204;
    (t37228, t37266, t37375, t37419, t37423, t37439, t37536, t37544, t37558, t37560)
}
