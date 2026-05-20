//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1181/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1181<F: Float>(t126125: F, t32474: F, t4469: F, t8477: F, t31844: F, t826: F, t126046: F, t247: F, t31752: F, t4366: F, t126250: F, t31805: F) -> (F, F, F, F) {
    let t126271 = t32474 * t126125;
    let t126273 = t8477 * t4469;
    let t126276 = t31844 * t826;
    let t126280 = t31752 * t126276 * t247 * t126046 * t4366;
    let t126282 = t31805 * t126250;
    (t126271, t126273, t126280, t126282)
}
