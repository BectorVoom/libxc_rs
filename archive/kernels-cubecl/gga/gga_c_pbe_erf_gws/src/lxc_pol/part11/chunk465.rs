//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 465/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk465<F: Float>(t506: F, t9: F, t1076: F, t169: F, t301: F, t784: F, t285: F, t545: F, t991: F, t281: F, t1083: F, t751: F) -> (F, F, F, F, F) {
    let t2912 = t9 * t506;
    let t2926 = t169 * t784 * t1076 * t301;
    let t2936 = t991 * t545 * t285;
    let t2937 = t281 * t2936;
    let t2939 = t751 * t1083;
    (t2912, t2926, t2936, t2937, t2939)
}
