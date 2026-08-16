//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2106/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2106(t27628: f64, t27634: f64, t10469: f64, t24719: f64, t3: f64, t86154: f64, t2132: f64, t24746: f64, t95382: f64, t1222: f64, t27589: f64, t1184: f64, t1409: f64) -> (f64, f64, f64, f64, f64) {
    let t95387 = t27634 * t27628;
    let t95396 = t86154 * t3 * t24719 * t10469;
    let t95404 = 0.20186378047070195428e-3_f64 * t2132 * t95382 * t24746;
    let t95410 = t27589 * t1222 / 216.0_f64;
    let t95413 = t1409 * t1184;
    (t95387, t95396, t95404, t95410, t95413)
}
