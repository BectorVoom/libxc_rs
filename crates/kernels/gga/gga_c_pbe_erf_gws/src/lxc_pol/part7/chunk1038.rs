//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1038/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1038<F: Float>(t6698: F, t8967: F, t6316: F, t6627: F, t2189: F, t6241: F, t3139: F, t6177: F, t8903: F, t6310: F, t6484: F, t6530: F, t20296: F, t2168: F, t2170: F, t2171: F) -> (F, F, F, F, F, F, F) {
    let t20847 = t8967 * t6698;
    let t20848 = 7.0 / 6.0 * t20847;
    let t20849 = t6627 * t6316;
    let t20851 = t6241 * t2189;
    let t20855 = 3.0 / 8.0 * t8903 * t3139 * t6177 * t20851;
    let t20856 = t6627 * t6310;
    let t20858 = t6484 * t6530;
    let t20859 = 7.0 / 12.0 * t20858;
    let t20863 = t2168 * t2170 * t20296 * t2171 / 12.0;
    (t20848, t20849, t20851, t20855, t20856, t20859, t20863)
}
