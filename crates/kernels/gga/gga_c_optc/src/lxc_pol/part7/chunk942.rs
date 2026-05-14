//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 942/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk942<F: Float>(t22265: F, t6881: F, t21874: F, t21878: F, t21880: F, t21884: F, t21887: F, t21891: F, t21895: F, t21899: F, t21903: F, t21907: F, t21932: F, t22069: F, t22071: F, t22074: F, t22079: F, t22082: F, t22091: F, t22093: F, t22098: F, t22103: F, t22107: F, t22111: F, t22115: F, t22117: F, t22119: F) -> (F, F, F) {
    let t22266 = t22265 * t6881;
    let t22268 = -t21874 - t21878 + t21880 + t21884 + t21887 + t21891 + t21895 - t21899 - t21903 - t21907 + t21932 + t22069;
    let t22269 = t22071 - t22074 + t22079 + t22082 + t22091 - t22093 - t22098 - t22103 + t22107 + t22111 + t22115 - t22117 - t22119;
    (t22266, t22268, t22269)
}
