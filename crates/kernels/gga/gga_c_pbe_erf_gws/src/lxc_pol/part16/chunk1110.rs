//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1110/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1110<F: Float>(t14011: F, t9358: F, t9406: F, t14007: F, t9443: F, t14015: F, t9470: F, t9366: F, t14093: F, t8848: F, t1135: F, t9246: F, t2134: F, t1125: F, t51317: F, t4039: F, t9411: F) -> (F, F, F, F, F, F, F, F, F) {
    let t54059 = t14011 * t9358;
    let t54061 = t14011 * t9406;
    let t54063 = t14007 * t9443;
    let t54065 = t14015 * t9470;
    let t54067 = t14007 * t9366;
    let t54069 = t8848 * t14093;
    let t54071 = t9246 * t1135;
    let t54072 = t2134 * t54071;
    let t54075 = t1125 * t51317;
    let t54077 = t4039 * t9411;
    (t54059, t54061, t54063, t54065, t54067, t54069, t54072, t54075, t54077)
}
