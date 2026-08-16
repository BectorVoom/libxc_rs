//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1472/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1472(t15281: f64, t4936: f64, t1174: f64, t3431: f64, t4912: f64, t1090: f64, t7319: f64, t4919: f64, t11531: f64, t11534: f64, t11537: f64, t11541: f64, t11591: f64, t15265: f64, t15269: f64, t15274: f64, t15278: f64, t3447: f64) -> f64 {
    let t15282 = t15281 * t4936;
    let t15284 = 0.55555555555555555554e-3_f64 * t1174 * t15282;
    let t15285 = t3431 * t4912;
    let t15287 = 0.18518518518518518518e-3_f64 * t1174 * t15285;
    let t15288 = t7319 * t1090;
    let t15289 = t4919 * t15288;
    let t15292 = 0.12345679012345679012e-3_f64 * t11531 - 0.9259259259259259259e-4_f64 * t11534 - 0.18518518518518518518e-3_f64 * t11537 + 0.12345679012345679012e-3_f64 * t11541 + 0.18518518518518518518e-3_f64 * t11591 + 0.49382716049382716049e-3_f64 * t15265 - 0.16666666666666666666e-2_f64 * t1174 * t15269 - 0.83333333333333333332e-3_f64 * t1174 * t15274 - 0.27777777777777777777e-3_f64 * t1174 * t15278 - t15284 - t15287 + 0.55555555555555555554e-3_f64 * t3447 * t15289;
    t15292
}
