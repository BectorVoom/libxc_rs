//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1149/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1149<F: Float>(t50765: F, t893: F, t10856: F, t16961: F, t2668: F, t10894: F, t16984: F, t2640: F, t16644: F, t8152: F, t862: F, t16990: F, t7386: F, t888: F) -> (F, F, F, F, F) {
    let t50766 = t893 * t50765;
    let t50823 = t2668 * t10856 * t16961;
    let t50828 = t2640 * t10894 * t16984;
    let t50869 = t862 * t8152 * t16644;
    let t50874 = t7386 * t888 * t16990;
    (t50766, t50823, t50828, t50869, t50874)
}
