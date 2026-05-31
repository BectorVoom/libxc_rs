//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3258/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3258<F: Float>(t1913: F, t4168: F, t18217: F, t571: F, t1921: F, t4153: F, t1464: F, t5789: F, t18177: F, t575: F, t13226: F, t13250: F, t1456: F, t1458: F, t18178: F, t1914: F, t3: F, t39397: F, t39399: F, t39401: F, t39403: F, t4154: F, t47730: F, t5790: F, t5808: F, t60560: F, t60599: F) -> F {
    let t60607 = t1913 * t4168;
    let t60609 = t571 * t18217;
    let t60611 = t4153 * t1921;
    let t60616 = t5789 * t1464;
    let t60618 = t18177 * t575;
    let tv4rho41 = t3 * t575 * t60560 + t13226 * t1921 + t13250 * t1914 + F::cast_from(3.0_f64) * t1456 * t18217 + t1458 * t60599 + F::cast_from(3.0_f64) * t1464 * t18178 + F::cast_from(3.0_f64) * t4154 * t5808 + F::cast_from(3.0_f64) * t4168 * t5790 + t39397 + F::cast_from(3.0_f64) * t39399 + F::cast_from(3.0_f64) * t39401 + t39403 + F::cast_from(6.0_f64) * t47730 + F::cast_from(3.0_f64) * t60607 + F::cast_from(3.0_f64) * t60609 + F::cast_from(3.0_f64) * t60611 + F::cast_from(6.0_f64) * t60616 + F::cast_from(3.0_f64) * t60618;
    tv4rho41
}
