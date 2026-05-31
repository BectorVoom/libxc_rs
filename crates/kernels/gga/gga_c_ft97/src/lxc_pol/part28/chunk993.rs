//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 993/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk993<F: Float>(t7397: F, t8232: F, t33193: F, t8392: F, t604: F, t7339: F, t139320: F, t139323: F, t139492: F, t139495: F, t139533: F, t1882: F, t33077: F) -> (F, F, F, F, F, F, F, F, F) {
    let t139896 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t8232 * t7397;
    let t139940 = t8392 * t33193;
    let t139950 = t604 * t7339;
    let t139991 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t139320;
    let t139992 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t139323;
    let t140041 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t139492;
    let t140042 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t139495;
    let t140053 = F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t139533;
    let t140068 = t1882 * t33077;
    (t139896, t139940, t139950, t139991, t139992, t140041, t140042, t140053, t140068)
}
