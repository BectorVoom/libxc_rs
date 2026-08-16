//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 114/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk114(t322: f64, t330: f64, t333: f64, t335: f64, t337: f64, t339: f64, t341: f64, t343: f64, t352: f64, t245: f64) -> (f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t354 = piecewise5(t323, t330, t331, 1.0_f64 - 0.64e0_f64 * t333 - 0.4352e0_f64 * t335 - 0.1535685604549e1_f64 * t337 + 0.3061560252175e1_f64 * t339 - 0.1915710236206e1_f64 * t341 + 0.516884468372e0_f64 * t343 - 0.51848879792e-1_f64 * t339 * t337, -0.7e0_f64 * t352);
    let t357 = f64::exp(1.0_f64 * t245);
    (t354, t357)
}
