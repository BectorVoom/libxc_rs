//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1637/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1637(t22393: f64, t22418: f64, t22430: f64, t22459: f64, t1343: f64, t1353: f64, t13599: f64, t13600: f64, t1450: f64, t1868: f64, t198: f64, t21901: f64, t21905: f64, t21933: f64, t21937: f64, t21969: f64, t4139: f64, t532: f64, t5532: f64, t5536: f64, t5591: f64, t5627: f64, t9278: f64, t9308: f64, t9316: f64, t9320: f64, t9325: f64, t9329: f64, t9333: f64, t9374: f64, t9389: f64, t9391: f64) -> (f64, f64) {
    let t22461 = t22393 + t22418 + t22430 + t22459;
    let t22465 = t1450 * t198 * t22461 * t532 + 3.0_f64 * t1343 * t198 * t21969 + 3.0_f64 * t1353 * t21937 * t4139 + 6.0_f64 * t13600 * t1868 * t4139 + 6.0_f64 * t4139 * t5532 * t5591 + 12.0_f64 * t5532 * t5536 * t5627 - t13599 + t21901 - t21905 + t21933 - t9278 + t9308 + t9316 + t9320 - t9325 + t9329 + t9333 - t9374 - t9389 - t9391;
    (t22461, t22465)
}
