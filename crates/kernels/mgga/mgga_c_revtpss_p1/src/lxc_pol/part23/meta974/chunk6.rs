//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3316/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3316<F: Float>(t1456: F, t1458: F, t1464: F, t1914: F, t1921: F, t22533: F, t22571: F, t25049: F, t25072: F, t3: F, t575: F, t5790: F, t5808: F, t60620: F, t60624: F, t60629: F, t6937: F, t6951: F, t75720: F, t75727: F, t75796: F, t75808: F, t86893: F, t86897: F, t86903: F, t86909: F, t86958: F) -> F {
    let tv4rho43 = t3 * t575 * t86893 + t1456 * t25072 + t1458 * t86958 + t1464 * t25049 + F::cast_from(3.0_f64) * t1914 * t22571 + F::cast_from(3.0_f64) * t1921 * t22533 + F::cast_from(3.0_f64) * t5790 * t6951 + F::cast_from(3.0_f64) * t5808 * t6937 + F::cast_from(6.0_f64) * t60620 + F::cast_from(6.0_f64) * t60624 + F::cast_from(3.0_f64) * t60629 + F::cast_from(3.0_f64) * t75720 + F::cast_from(3.0_f64) * t75727 + F::cast_from(3.0_f64) * t75796 + t75808 + F::cast_from(3.0_f64) * t86897 + F::cast_from(3.0_f64) * t86903 + t86909;
    tv4rho43
}
