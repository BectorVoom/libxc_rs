//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 739/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk739<F: Float>(t11: F, t340: F, t14: F, t15564: F, t81: F, t8633: F, t2984: F, t2258: F, t2993: F, t1528: F, t18: F, t342: F, t4410: F, t630: F) -> (F, F, F, F, F) {
    let t15565 = t340 * t11;
    let t15567 = t15564 * t15565 * t14;
    let t15568 = t8633 * t81;
    let t15569 = t15568 * t2984;
    let t15575 = t2258 * t81;
    let t15576 = t15575 * t2993;
    let t15579 = t1528 * t18;
    let t15584 = t342 * t630 * t4410;
    (t15567, t15569, t15576, t15579, t15584)
}
