//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1316/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1316<F: Float>(t14535: F, t3120: F, t11606: F, t14015: F, t3123: F, t54084: F, t11901: F, t14011: F, t14046: F, t15268: F, t11620: F, t3139: F, t37441: F, t4028: F) -> (F, F, F, F, F, F, F) {
    let t57060 = t3120 * t14535;
    let t57062 = t14015 * t11606;
    let t57064 = t3123 * t54084;
    let t57066 = t14011 * t11901;
    let t57068 = t14046 * t15268;
    let t57070 = t14011 * t11620;
    let t57073 = t4028 * t3139 * t37441;
    (t57060, t57062, t57064, t57066, t57068, t57070, t57073)
}
