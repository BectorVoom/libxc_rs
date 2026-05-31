//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2226/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2226<F: Float>(t670: F, t8151: F, t104115: F, t109012: F, t109014: F, t109024: F, t109029: F, t109035: F, t109038: F, t109039: F, t1519: F, t18235: F, t1911: F, t2163: F, t21881: F, t2322: F, t29427: F, t29437: F, t29459: F, t30944: F, t30951: F, t4248: F, t4254: F, t4257: F, t4293: F, t5920: F, t651: F, t7586: F, t7683: F) -> (F, F) {
    let t111734 = t8151 * t670;
    let t111746 = -F::cast_from(2.0_f64) * t2163 * t21881 * t651 - F::cast_from(2.0_f64) * t30944 * t651 * t670 - F::cast_from(2.0_f64) * t5920 * t651 * t7683 - F::cast_from(4.0_f64) * t104115 * t1519 - F::cast_from(4.0_f64) * t111734 * t1519 - F::cast_from(4.0_f64) * t18235 * t7586 + F::cast_from(2.0_f64) * t1911 * t29437 - F::cast_from(2.0_f64) * t2322 * t30951 - F::cast_from(4.0_f64) * t29427 * t4257 - F::cast_from(4.0_f64) * t29427 * t4293 - F::cast_from(4.0_f64) * t29459 * t4248 - F::cast_from(2.0_f64) * t30951 * t4254 - t109012 + t109014 - t109024 - t109029 - t109035 - t109038 - t109039;
    (t111734, t111746)
}
