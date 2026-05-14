//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 501/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk501<F: Float>(t1969: F, t3338: F, t446: F, t1017: F, t1984: F, t558: F, t28: F, t89: F, t132: F, t538: F, t1009: F, t1995: F, t1008: F, t549: F, t554: F, t2007: F, t929: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3339 = t1969 * t3338;
    let t3340 = t446 * t3339;
    let t3342 = t1984 * t1017;
    let t3343 = t3342 * t558;
    let t3345 = t89 * t28 * t3343;
    let t3347 = t538 * t132;
    let t3348 = t3347 * t1009;
    let t3350 = t1995 * t1009;
    let t3355 = t549 * t1008;
    let t3356 = t3355 * t554;
    let t3359 = t2007 * t929;
    (t3339, t3340, t3342, t3343, t3345, t3347, t3348, t3350, t3355, t3356, t3359)
}
