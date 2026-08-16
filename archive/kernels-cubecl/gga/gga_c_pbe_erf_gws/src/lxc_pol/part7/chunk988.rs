//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 988/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk988<F: Float>(t16884: F, t16889: F, t16891: F, t16893: F, t16896: F, t16900: F, t16902: F, t16907: F, t16910: F, t16912: F, t16917: F, t16921: F) -> F {
    let t18204 = -t16884 + t16889 - t16891 - t16893 + t16896 - t16900 - t16902 - t16907 - t16910 - t16912 - t16917 + t16921;
    t18204
}
