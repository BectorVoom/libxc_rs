//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2226/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2226(t670: f64, t8151: f64, t104115: f64, t109012: f64, t109014: f64, t109024: f64, t109029: f64, t109035: f64, t109038: f64, t109039: f64, t1519: f64, t18235: f64, t1911: f64, t2163: f64, t21881: f64, t2322: f64, t29427: f64, t29437: f64, t29459: f64, t30944: f64, t30951: f64, t4248: f64, t4254: f64, t4257: f64, t4293: f64, t5920: f64, t651: f64, t7586: f64, t7683: f64) -> (f64, f64) {
    let t111734 = t8151 * t670;
    let t111746 = -2.0_f64 * t2163 * t21881 * t651 - 2.0_f64 * t30944 * t651 * t670 - 2.0_f64 * t5920 * t651 * t7683 - 4.0_f64 * t104115 * t1519 - 4.0_f64 * t111734 * t1519 - 4.0_f64 * t18235 * t7586 + 2.0_f64 * t1911 * t29437 - 2.0_f64 * t2322 * t30951 - 4.0_f64 * t29427 * t4257 - 4.0_f64 * t29427 * t4293 - 4.0_f64 * t29459 * t4248 - 2.0_f64 * t30951 * t4254 - t109012 + t109014 - t109024 - t109029 - t109035 - t109038 - t109039;
    (t111734, t111746)
}
