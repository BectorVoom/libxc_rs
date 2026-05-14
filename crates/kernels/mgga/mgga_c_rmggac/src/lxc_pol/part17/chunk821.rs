//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 821/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk821<F: Float>(t34884: F, t9990: F, t10095: F, t16156: F, t10082: F, t333: F, t3351: F, t511: F, t7248: F, t38530: F, t9159: F, t34975: F, t34976: F, t571: F, t8455: F, t1368: F, t16503: F, t3369: F, t9163: F) -> (F, F, F, F, F, F) {
    let t45486 = t34884 * t9990;
    let t45488 = t16156 * t10095;
    let t45493 = t3351 * t7248 * t511 * t10082 * t333;
    let t45495 = t38530 * t9159;
    let t45499 = t34975 * t34976 * t571 * t8455;
    let t45503 = t16503 * t3369 * t1368 * t9163;
    (t45486, t45488, t45493, t45495, t45499, t45503)
}
