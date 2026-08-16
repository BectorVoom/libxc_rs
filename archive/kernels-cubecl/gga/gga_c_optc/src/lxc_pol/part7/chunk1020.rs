//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1020/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1020<F: Float>(t21874: F, t21878: F, t21880: F, t21884: F, t21887: F, t21891: F, t21895: F, t21899: F, t21903: F, t21907: F, t21932: F, t22069: F) -> F {
    let t22268 = -t21874 - t21878 + t21880 + t21884 + t21887 + t21891 + t21895 - t21899 - t21903 - t21907 + t21932 + t22069;
    t22268
}
