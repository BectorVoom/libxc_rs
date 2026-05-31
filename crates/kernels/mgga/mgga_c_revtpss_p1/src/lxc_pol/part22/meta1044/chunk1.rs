//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3655/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3655<F: Float>(t43865: F, t43888: F, t43890: F, t43892: F, t56230: F, t56236: F, t68389: F, t68393: F, t68397: F, t68399: F, t68454: F, t68456: F, t68459: F) -> F {
    let t69086 = -F::cast_from(0.11872222222222222222e-1_f64) * t56230 - F::cast_from(0.36935802469135802468e-1_f64) * t56236 - F::cast_from(0.11872222222222222222e-1_f64) * t68389 + F::cast_from(0.17808333333333333333e-1_f64) * t68393 - F::cast_from(0.23744444444444444444e-1_f64) * t68397 + F::cast_from(0.15829629629629629629e-1_f64) * t68399 - F::cast_from(0.52765432098765432098e-2_f64) * t43865 - F::cast_from(0.36935802469135802468e-1_f64) * t43888 + F::cast_from(0.79148148148148148147e-2_f64) * t43890 + F::cast_from(0.15829629629629629629e-1_f64) * t43892 - F::cast_from(0.47488888888888888888e-1_f64) * t68454 - F::cast_from(0.71233333333333333333e-1_f64) * t68456 + F::cast_from(0.10685e0_f64) * t68459;
    t69086
}
