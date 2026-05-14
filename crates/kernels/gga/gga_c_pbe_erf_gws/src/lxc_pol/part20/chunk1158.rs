//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1158/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1158<F: Float>(t11423: F, t51351: F, t3116: F, t54373: F, t3065: F, t3840: F, t6645: F, t3879: F, t2134: F, t3759: F, t51214: F, t11516: F, t14011: F, t11934: F, t51222: F, t54053: F, t54073: F, t54088: F, t55469: F, t56910: F) -> (F,) {
    let t56912 = t51351 * t11423;
    let t56914 = t3116 * t54373;
    let t56916 = t3065 * t3840;
    let t56917 = t6645 * t56916;
    let t56919 = t3065 * t3879;
    let t56920 = t2134 * t56919;
    let t56922 = t51214 * t3759;
    let t56924 = t14011 * t11516;
    let t56926 = t14011 * t11934;
    let t56928 = 35.0 / 432.0 * t51222 + t56910 / 48.0 - t54053 + t56912 / 192.0 + t56914 / 24.0 + t54073 + t56917 / 48.0 - t56920 / 96.0 + 7.0 / 1152.0 * t56922 + t54088 + t55469 + t56924 / 192.0 - t56926 / 768.0;
    (t56928,)
}
