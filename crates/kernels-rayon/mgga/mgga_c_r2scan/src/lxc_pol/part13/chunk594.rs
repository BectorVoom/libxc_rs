//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 594/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk594(t792: f64, t797: f64, t3275: f64, t3276: f64, t503: f64, t57: f64) -> (f64, f64, f64) {
    let t3277 = t797 * t792;
    let t3279 = t3275 * t3276 * t3277;
    let t3280 = 5.0_f64 / 16.0_f64 * t3279;
    let t3281 = t503 * t57;
    (t3277, t3280, t3281)
}
