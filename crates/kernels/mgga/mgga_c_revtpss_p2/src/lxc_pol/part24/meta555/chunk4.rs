//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1660/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1660<F: Float>(t141: F, t2908: F, t88128: F, t41246: F, t77499: F, t77505: F, t77507: F, t77509: F, t77663: F, t77667: F, t88089: F, t88097: F, t88144: F, t88147: F, t88150: F, t88161: F) -> (F, F) {
    let t88164 = t141 * t2908 * t88128;
    let t88166 = -F::cast_from(0.8585111111111111111e-1_f64) * t88144 - F::cast_from(0.82785e-1_f64) * t88147 + F::cast_from(0.44152e0_f64) * t88150 - F::cast_from(0.44152e0_f64) * t77663 + F::cast_from(0.98115555555555555555e-1_f64) * t77667 - F::cast_from(0.108693e2_f64) * t88089 + F::cast_from(0.24154e1_f64) * t88097 + t41246 + F::cast_from(0.44729629629629629629e0_f64) * t77499 + F::cast_from(0.40256666666666666668e0_f64) * t77505 - F::cast_from(0.16102666666666666667e1_f64) * t77507 + F::cast_from(0.24154e1_f64) * t77509 - F::cast_from(0.99342e0_f64) * t88161 - F::cast_from(0.82785e-1_f64) * t88164;
    (t88164, t88166)
}
