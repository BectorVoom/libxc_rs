//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 667/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk667<F: Float>(t136: F, t2079: F, t634: F, t108: F, t6567: F, t117: F, t56: F, t104: F, t137: F, t131: F, t6165: F, t130: F, t142: F, t3440: F, t6: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6892 = t2079 * t136;
    let t6893 = t634 * t6892;
    let t6896 = t108 * t6567;
    let t6899 = 455.0 / 1296.0 * t6896 * t56 * t117;
    let t6915 = t137 * t104;
    let t6916 = 1.0 / t6915;
    let t6917 = t136 * t6916;
    let t6922 = t131 * t6165;
    let t6923 = t130 * t6922;
    let t6925 = 0.47892880429854730775e0 * t6923 * t142;
    let t6931 = t3440 * t6;
    (t6892, t6893, t6896, t6899, t6916, t6917, t6922, t6923, t6925, t6931)
}
