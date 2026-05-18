//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 929/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk929<F: Float>(t10014: F, t35637: F, t34884: F, t9990: F, t10095: F, t16156: F, t10082: F, t333: F, t3351: F, t511: F, t7248: F, t38530: F, t9159: F) -> (F, F, F, F, F) {
    let t45484 = t35637 * t10014;
    let t45486 = t34884 * t9990;
    let t45488 = t16156 * t10095;
    let t45493 = t3351 * t7248 * t511 * t10082 * t333;
    let t45495 = t38530 * t9159;
    (t45484, t45486, t45488, t45493, t45495)
}
