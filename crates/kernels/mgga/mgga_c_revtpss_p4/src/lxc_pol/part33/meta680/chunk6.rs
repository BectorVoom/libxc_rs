//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2220/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2220<F: Float>(t5842: F, t60: F, t104379: F, t108952: F, t18281: F, t1923: F, t1927: F, t19661: F, t19666: F, t19680: F, t2123: F, t26776: F, t28089: F, t28093: F, t29355: F, t29363: F, t29364: F, t29367: F, t29372: F, t29375: F, t30682: F, t30683: F, t30686: F, t4181: F, t4186: F, t606: F, t6954: F, t6977: F, t72: F, t7571: F, t7702: F, t7719: F, t8143: F, t8147: F, t92612: F, t96733: F) -> F {
    let t111592 = t5842 * t60;
    let t111623 = -t108952 * t2123 / F::cast_from(6.0_f64) - t7702 * t29364 / F::cast_from(3.0_f64) - t7702 * t29367 / F::cast_from(3.0_f64) - t28093 * t8147 / F::cast_from(3.0_f64) - t7702 * t29372 / F::cast_from(3.0_f64) - t7702 * t29375 / F::cast_from(3.0_f64) - t6954 * t30683 / F::cast_from(6.0_f64) - t1923 * (-F::cast_from(220.0_f64) / F::cast_from(27.0_f64) * t111592 * t606 - F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t104379 * t4181 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t29355 * t4186 + F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t96733 * t19661 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t26776 * t19666 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t26776 * t19680 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7571 * t18281 + t92612) * t72 * t1927 / F::cast_from(6.0_f64) - t1923 * t30682 * t6977 / F::cast_from(6.0_f64) - t6954 * t30686 / F::cast_from(3.0_f64) - t1923 * t29363 * t7719 / F::cast_from(3.0_f64) - t1923 * t8143 * t28089 / F::cast_from(3.0_f64);
    t111623
}
