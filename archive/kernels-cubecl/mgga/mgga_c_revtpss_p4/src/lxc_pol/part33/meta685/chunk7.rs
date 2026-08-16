//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2270/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2270<F: Float>(t30993: F, t571: F, t104094: F, t111419: F, t113015: F, t113019: F, t113022: F, t1456: F, t1464: F, t1921: F, t29469: F, t3: F, t30975: F, t575: F, t5790: F, t5808: F, t6937: F, t6951: F, t7691: F, t7700: F, t8241: F, t8249: F) -> F {
    let t113025 = t571 * t30993;
    let t113026 = t113015 * t3 * t575 + t1456 * t30993 + t1464 * t30975 + F::cast_from(2.0_f64) * t1921 * t29469 + F::cast_from(2.0_f64) * t5790 * t8249 + F::cast_from(2.0_f64) * t5808 * t8241 + t6937 * t7700 + t6951 * t7691 + t104094 + t111419 + t113019 + F::cast_from(2.0_f64) * t113022 + t113025;
    t113026
}
