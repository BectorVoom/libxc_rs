//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1203/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1203(t39358: f64, t39361: f64, t39352: f64, t39364: f64, t39367: f64, t39370: f64, t39373: f64, t39376: f64, t39379: f64, t39381: f64, t41352: f64, t39395: f64) -> (f64, f64) {
    let t41353 = 0.11426392607441748234e0_f64 * t39358;
    let t41354 = 0.46230515946956099004e0_f64 * t39361;
    let t41362 = -0.32927245914677557992e0_f64 * t39352 - t41352 - t41353 - t41354 + 0.86682217400542685632e-1_f64 * t39364 + 0.2600466522016280569e0_f64 * t39367 - 0.17336443480108537126e0_f64 * t39370 - 0.17336443480108537126e0_f64 * t39373 + 0.17336443480108537126e0_f64 * t39376 + 0.5200933044032561138e0_f64 * t39379 + 0.17336443480108537126e0_f64 * t39381;
    let t41367 = 0.25610080155860322884e0_f64 * t39395;
    (t41362, t41367)
}
