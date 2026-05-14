//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 86/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk86<F: Float>(t13: F, t14: F, t12: F, t18: F, t19: F, t17: F) -> (F, F, F, F, F, F) {
    let t349 = t14 * t13;
    let t350 = 1.0 / t349;
    let t351 = t12 * t350;
    let t360 = t13 * t13;
    let t361 = 1.0 / t360;
    let t362 = t18 * t361;
    let t363 = t19 - t362;
    let t375 = t350 * t17;
    (t350, t351, t360, t362, t363, t375)
}
