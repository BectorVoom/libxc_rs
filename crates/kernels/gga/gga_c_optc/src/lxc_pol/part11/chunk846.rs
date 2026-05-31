//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 846/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk846<F: Float>(t9707: F, t9715: F, t13111: F, t13114: F, t9721: F, t6480: F, t6484: F, t6488: F, t6492: F, t6816: F, t6823: F, t6827: F, t6840: F) -> (F, F, F, F, F, F) {
    let t16344 = F::cast_from(0.73246220147012639764e-3_f64) * t9707;
    let t16345 = F::cast_from(24.0_f64) * t9715;
    let t16346 = F::cast_from(3.0_f64) * t13111;
    let t16347 = F::cast_from(0.54934665110259479823e-3_f64) * t13114;
    let t16348 = F::cast_from(24.0_f64) * t9721;
    let t16349 = t16344 + t6816 - t16345 - t6480 - t6484 + t6488 - t6823 + t6827 + t16346 - t16347 - t16348 + t6492 - t6840;
    (t16344, t16345, t16346, t16347, t16348, t16349)
}
