//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1067/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1067<F: Float>(t2148: F, t7073: F, t146: F, t147: F, t6567: F, t688: F, t2152: F, t155: F, t158: F, t6165: F, t661: F, t2144: F, t7083: F) -> (F, F, F, F, F) {
    let t23160 = t7073 * t2148;
    let t23163 = t146 * t147 * t6567;
    let t23164 = t23163 * t688;
    let t23166 = t7073 * t2152;
    let t23171 = t155 * t158 * t6165;
    let t23172 = t23171 * t661;
    let t23174 = t2144 * t7083;
    (t23160, t23164, t23166, t23172, t23174)
}
