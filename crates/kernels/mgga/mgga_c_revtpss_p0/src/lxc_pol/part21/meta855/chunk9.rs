//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3244/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3244<F: Float>(t10416: F, t118: F, t13207: F, t13521: F, t13532: F, t13540: F, t1502: F, t1519: F, t18153: F, t18163: F, t2322: F, t3813: F, t4246: F, t4254: F, t4257: F, t4292: F, t46126: F, t49851: F, t49856: F, t56137: F, t60177: F, t651: F, t670: F) -> F {
    let t60183 = -F::cast_from(6.0_f64) * t2322 * t13521 - F::cast_from(6.0_f64) * t651 * t18153 * t670 - F::cast_from(12.0_f64) * t2322 * t13532 - F::cast_from(12.0_f64) * t4254 * t13532 - F::cast_from(6.0_f64) * t651 * t3813 * t4292 - F::cast_from(12.0_f64) * t2322 * t13540 - F::cast_from(2.0_f64) * t46126 * t1519 - F::cast_from(6.0_f64) * t49851 * t1519 - F::cast_from(6.0_f64) * t10416 * t4257 - F::cast_from(2.0_f64) * t49856 * t1519 - F::cast_from(6.0_f64) * t18163 * t4257 - t118 * (t56137 + t60177) - F::cast_from(3.0_f64) * t4246 * t3813 - t1502 * t13207;
    t60183
}
