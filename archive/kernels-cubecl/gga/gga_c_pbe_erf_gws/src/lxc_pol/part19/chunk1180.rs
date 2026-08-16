//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1180/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1180<F: Float>(t14092: F, t3792: F, t14538: F, t3857: F, t4043: F, t14011: F, t3816: F, t11776: F, t3139: F, t4028: F, t3871: F, t4049: F) -> (F, F, F, F, F, F, F) {
    let t15240 = t14092 * t3792;
    let t15241 = t14538 * t15240;
    let t15243 = t4043 * t3857;
    let t15245 = t14011 * t3816;
    let t15248 = t3139 * t11776;
    let t15249 = t4028 * t15248;
    let t15251 = t4049 * t3871;
    (t15240, t15241, t15243, t15245, t15248, t15249, t15251)
}
