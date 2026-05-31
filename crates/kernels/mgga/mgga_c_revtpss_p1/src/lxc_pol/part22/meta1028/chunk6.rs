//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3610/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3610<F: Float>(t43865: F, t43888: F, t43890: F, t43892: F, t56230: F, t56236: F, t68389: F, t68393: F, t68397: F, t68399: F, t68454: F, t68456: F, t68459: F) -> F {
    let t68461 = -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t56230 - F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t56236 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t68389 + t68393 / F::cast_from(3.0_f64) - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t68397 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t68399 - F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t43865 - F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t43888 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t43890 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t43892 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t68454 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t68456 + F::cast_from(2.0_f64) * t68459;
    t68461
}
