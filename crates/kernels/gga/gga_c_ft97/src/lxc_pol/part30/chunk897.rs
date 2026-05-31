//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 897/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk897<F: Float>(t231: F, t36791: F, t1100: F, t17986: F, t1416: F, t7447: F, t1410: F, t6: F, t674: F, t7513: F, t7639: F, t797: F) -> (F, F, F, F, F, F) {
    let t36792 = t36791 * t231;
    let t36796 = t1100 * t17986;
    let t36801 = t7447 * t1416;
    let t36835 = t1410 * t6;
    let t36867 = F::cast_from(1.0_f64) / t7513 / t674;
    let t37041 = F::cast_from(1.0_f64) / t7639 / t797;
    (t36792, t36796, t36801, t36835, t36867, t37041)
}
