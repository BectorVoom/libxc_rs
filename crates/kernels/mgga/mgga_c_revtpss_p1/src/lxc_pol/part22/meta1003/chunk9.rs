//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3427/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3427<F: Float>(t52033: F, t52035: F, t52037: F, t52039: F, t52041: F, t52045: F, t63359: F, t63361: F, t63366: F, t63369: F, t63371: F, t63374: F) -> F {
    let t64386 = -F::cast_from(0.23744444444444444444e-1_f64) * t63359 + F::cast_from(0.71233333333333333333e-1_f64) * t63361 + F::cast_from(0.71233333333333333332e-1_f64) * t63366 - F::cast_from(0.10685e0_f64) * t63369 - F::cast_from(0.47488888888888888888e-1_f64) * t63371 - F::cast_from(0.10685e0_f64) * t63374 + F::cast_from(0.71233333333333333332e-1_f64) * t52033 + F::cast_from(0.63318518518518518517e-1_f64) * t52035 - F::cast_from(0.21106172839506172839e-1_f64) * t52037 - F::cast_from(0.47488888888888888888e-1_f64) * t52039 - F::cast_from(0.23744444444444444444e-1_f64) * t52041 - F::cast_from(0.47488888888888888888e-1_f64) * t52045;
    t64386
}
