//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 424/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk424(t2086: f64, t254: f64, t39: f64, t9: f64, t120: f64, t122: f64, t135: f64, t57: f64, t269: f64) -> (f64, f64, f64, f64) {
    let t2088 = 0.42377972951376424087e0_f64 * t254 * t2086;
    let t2090 = 1.0_f64 / t9 / t39;
    let t2095 = 0.21341733463216935736e0_f64 * t120 * t122 * t2090 * t57 * t135;
    let t2096 = t269 * t269;
    (t2088, t2090, t2095, t2096)
}
