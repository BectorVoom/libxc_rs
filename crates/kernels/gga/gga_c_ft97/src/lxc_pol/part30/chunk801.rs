//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 801/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk801<F: Float>(t1416: F, t7447: F, t1410: F, t6: F, t674: F, t7513: F, t7639: F, t797: F, t1609: F, t1610: F, t1613: F, t19: F, t7: F, t11: F, t1690: F, t10051: F, t754: F) -> (F, F, F, F, F, F, F, F) {
    let t36801 = t7447 * t1416;
    let t36835 = t1410 * t6;
    let t36867 = 1.0 / t7513 / t674;
    let t37041 = 1.0 / t7639 / t797;
    let t37481 = t1613 * t1610 * t1609;
    let t37991 = t7 * t19;
    let t38176 = t1690 * t11;
    let t41402 = t754 * t10051;
    (t36801, t36835, t36867, t37041, t37481, t37991, t38176, t41402)
}
