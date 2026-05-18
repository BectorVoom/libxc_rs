//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 456/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk456<F: Float>(t1017: F, t358: F, t363: F, t1969: F, t446: F, t1984: F, t558: F, t28: F, t89: F, t132: F, t538: F, t1009: F) -> (F, F, F, F, F, F, F, F) {
    let t3337 = t1017 * t358;
    let t3338 = t3337 * t363;
    let t3339 = t1969 * t3338;
    let t3340 = t446 * t3339;
    let t3342 = t1984 * t1017;
    let t3343 = t3342 * t558;
    let t3345 = t89 * t28 * t3343;
    let t3347 = t538 * t132;
    let t3348 = t3347 * t1009;
    (t3338, t3339, t3340, t3342, t3343, t3345, t3347, t3348)
}
