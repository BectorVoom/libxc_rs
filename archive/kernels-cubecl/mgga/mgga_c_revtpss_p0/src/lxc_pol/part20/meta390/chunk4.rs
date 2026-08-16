//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1430/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1430<F: Float>(t41308: F, t41312: F, t41316: F, t41320: F, t41323: F, t41327: F, t41329: F, t41330: F, t41332: F, t41334: F, t41336: F, t11852: F, t159: F) -> (F, F) {
    let t41338 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t41308 + F::cast_from(8.0_f64) * t41312 - F::cast_from(12.0_f64) * t41316 + F::cast_from(2.0_f64) * t41320 + F::cast_from(8.0_f64) * t41323 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t41327 + t41329 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t41330 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t41332 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t41334 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t41336;
    let t41339 = t159 * t11852;
    (t41338, t41339)
}
