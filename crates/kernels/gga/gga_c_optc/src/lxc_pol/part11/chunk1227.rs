//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1227/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1227<F: Float>(t141: F, t2087: F, t55893: F, t1260: F, t16287: F, t55933: F, t659: F, t21874: F, t21878: F, t21884: F, t21887: F, t21891: F, t21895: F, t21899: F, t21903: F, t21907: F, t55862: F, t55875: F, t55878: F) -> (F, F, F, F) {
    let t56229 = t2087 * t141 * t55893;
    let t56232 = t1260 * t16287;
    let t56252 = t659 * t141 * t55933;
    let t56255 = t55862 - t21874 - t21878 + t21884 + t21887 + t21891 + t21895 - t21899 - t21903 - t21907 + t55875 + t55878;
    (t56229, t56232, t56252, t56255)
}
