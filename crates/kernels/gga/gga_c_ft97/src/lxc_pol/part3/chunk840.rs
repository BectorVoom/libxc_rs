//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 840/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk840<F: Float>(t3565: F, t3578: F, t144: F, t4668: F, t558: F, t167: F, t9432: F, t609: F, t2185: F, t605: F, t1017: F, t3408: F) -> (F, F, F, F, F) {
    let t17062 = t3578 * t3565;
    let t17063 = t144 * t17062;
    let t17066 = t4668 * t558;
    let t17068 = t9432 * t167 * t17066;
    let t17071 = t4668 * t609;
    let t17073 = t2185 * t605 * t17071;
    let t17076 = t1017 * t3408;
    (t17062, t17063, t17068, t17073, t17076)
}
