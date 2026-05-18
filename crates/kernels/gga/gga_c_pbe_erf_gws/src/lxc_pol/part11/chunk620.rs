//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 620/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk620<F: Float>(t1651: F, t597: F, t1630: F, t649: F, t596: F, t188: F, t108: F, t615: F, t267: F) -> (F, F, F, F, F, F, F) {
    let t5129 = t1651 * t597;
    let t5137 = t1630 * t649;
    let t5174 = t596 * t596;
    let t5175 = F::new(1.0) / t5174;
    let t5176 = t188 * t5175;
    let t5210 = t615 * t108;
    let t5211 = t5210 * t267;
    (t5129, t5137, t5174, t5175, t5176, t5210, t5211)
}
