//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3705/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3705<F: Float>(t43865: F, t43888: F, t43890: F, t43892: F, t56230: F, t56236: F, t68389: F, t68393: F, t68397: F, t68399: F, t68454: F, t68456: F, t68459: F) -> F {
    let t70200 = -F::cast_from(0.9877777777777777778e-2_f64) * t56230 - F::cast_from(0.30730864197530864199e-1_f64) * t56236 - F::cast_from(0.9877777777777777778e-2_f64) * t68389 + F::cast_from(0.14816666666666666667e-1_f64) * t68393 - F::cast_from(0.19755555555555555556e-1_f64) * t68397 + F::cast_from(0.13170370370370370371e-1_f64) * t68399 - F::cast_from(0.43901234567901234569e-2_f64) * t43865 - F::cast_from(0.30730864197530864198e-1_f64) * t43888 + F::cast_from(0.65851851851851851853e-2_f64) * t43890 + F::cast_from(0.13170370370370370371e-1_f64) * t43892 - F::cast_from(0.39511111111111111112e-1_f64) * t68454 - F::cast_from(0.59266666666666666668e-1_f64) * t68456 + F::cast_from(0.88900000000000000002e-1_f64) * t68459;
    t70200
}
