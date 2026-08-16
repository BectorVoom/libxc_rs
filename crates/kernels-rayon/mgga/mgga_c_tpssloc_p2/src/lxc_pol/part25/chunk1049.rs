//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1049/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1049(t22692: f64, t3851: f64, t7208: f64, t22717: f64, t22725: f64, t1332: f64, t1336: f64, t2089: f64, t22697: f64, t22701: f64, t22707: f64, t22721: f64, t22728: f64, t22730: f64, t3773: f64, t3777: f64, t7209: f64, t7211: f64) -> (f64, f64) {
    let t24099 = 0.16449340668482264365e-1_f64 * t22692;
    let t24103 = t7208 * t3851;
    let t24108 = 0.12793931631041761173e0_f64 * t22717;
    let t24110 = 0.52089578783527170489e-1_f64 * t22725;
    let t24115 = -t24099 + t3773 * t2089 + 2.0_f64 * t1332 * t7211 - t1336 * t24103 - 0.3289868133696452873e-1_f64 * t22697 - 0.16449340668482264365e-1_f64 * t22701 + 0.16449340668482264365e-1_f64 * t22707 + t24108 + 0.16449340668482264365e-1_f64 * t22721 + t24110 - 0.16449340668482264365e-1_f64 * t22728 - 0.76763589786250567036e-1_f64 * t22730 - 2.0_f64 * t3777 * t7209;
    (t24103, t24115)
}
