//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 989/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk989<F: Float>(t1235: F, t30339: F, t20292: F, t20373: F, t26138: F, t26150: F, t26159: F, t26176: F, t26179: F, t30288: F, t30292: F, t30296: F, t30300: F, t30303: F, t30327: F) -> (F, F) {
    let t30340 = t1235 * t30339;
    let t30350 = -F::cast_from(0.59793333333333333333e0_f64) * t30296 + F::cast_from(0.17938e1_f64) * t30303 - F::cast_from(0.5477111111111111111e0_f64) * t20373 - F::cast_from(0.39862222222222222223e0_f64) * t20292 - F::cast_from(0.76790625e-1_f64) * t30327 + F::cast_from(0.1898925e1_f64) * t30340 + F::cast_from(0.10954222222222222222e0_f64) * t26176 - F::cast_from(0.65725333333333333332e0_f64) * t26179 - F::cast_from(0.59793333333333333333e0_f64) * t26150 + F::cast_from(0.29896666666666666667e0_f64) * t26159 + F::cast_from(0.19931111111111111111e0_f64) * t26138 - F::cast_from(0.33218518518518518518e0_f64) * t30288 + F::cast_from(0.11958666666666666667e1_f64) * t30292 - F::cast_from(0.17938e1_f64) * t30300;
    (t30340, t30350)
}
