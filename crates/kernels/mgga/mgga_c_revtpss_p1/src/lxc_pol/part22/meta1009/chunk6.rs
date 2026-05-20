//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3458/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3458<F: Float>(t52033: F, t52035: F, t52037: F, t52039: F, t52041: F, t52045: F, t63359: F, t63361: F, t63366: F, t63369: F, t63371: F, t63374: F) -> F {
    let t65040 = -F::cast_from(0.11111111111111111111e-1_f64) * t63359 + F::cast_from(0.33333333333333333334e-1_f64) * t63361 + F::cast_from(0.33333333333333333333e-1_f64) * t63366 - F::cast_from(0.50000000000000000001e-1_f64) * t63369 - F::cast_from(0.22222222222222222222e-1_f64) * t63371 - F::cast_from(0.50000000000000000001e-1_f64) * t63374 + F::cast_from(0.33333333333333333333e-1_f64) * t52033 + F::cast_from(0.2962962962962962963e-1_f64) * t52035 - F::cast_from(0.98765432098765432098e-2_f64) * t52037 - F::cast_from(0.22222222222222222222e-1_f64) * t52039 - F::cast_from(0.11111111111111111111e-1_f64) * t52041 - F::cast_from(0.22222222222222222222e-1_f64) * t52045;
    t65040
}
