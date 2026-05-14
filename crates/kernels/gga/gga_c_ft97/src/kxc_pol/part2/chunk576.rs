//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 576/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk576<F: Float>(t26: F, t7943: F, t1771: F, t380: F, t1644: F, t458: F, t1648: F, t1652: F, t17: F, t7760: F, t1594: F, t7858: F, t62: F, t66: F, t1613: F, t77: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7944 = t26 * t7943;
    let t7945 = 28.0 / 27.0 * t7944;
    let t7946 = t1771 * t380;
    let t7948 = t458 * t1644;
    let t7950 = t458 * t1648;
    let t7952 = t458 * t1652;
    let t7954 = t17 * t7760;
    let t7982 = t1594 * t7858;
    let t7983 = t62 * t66;
    let t7998 = t77 * t1613;
    (t7944, t7945, t7946, t7948, t7950, t7952, t7954, t7982, t7983, t7998)
}
