//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3937/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3937<F: Float>(t1456: F, t1464: F, t22533: F, t22571: F, t3: F, t4154: F, t47730: F, t575: F, t60607: F, t60620: F, t60624: F, t60629: F, t6951: F, t75716: F, t75720: F, t75801: F) -> F {
    let tv4rho42 = t3 * t575 * t75716 + F::cast_from(2.0_f64) * t1456 * t22571 + F::cast_from(2.0_f64) * t1464 * t22533 + t4154 * t6951 + F::cast_from(4.0_f64) * t47730 + F::cast_from(2.0_f64) * t60607 + F::cast_from(4.0_f64) * t60620 + F::cast_from(4.0_f64) * t60624 + F::cast_from(2.0_f64) * t60629 + F::cast_from(2.0_f64) * t75720 + t75801;
    tv4rho42
}
