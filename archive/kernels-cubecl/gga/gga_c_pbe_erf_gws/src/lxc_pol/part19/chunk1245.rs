//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1245/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1245<F: Float>(t14046: F, t3172: F, t14565: F, t346: F, t838: F, t859: F, t4142: F, t51529: F, t13953: F, t14648: F, t51877: F, t13972: F, t14684: F) -> (F, F, F, F, F, F) {
    let t54397 = t14046 * t3172;
    let t54401 = t14565 * t346 * t838 * t859;
    let t54427 = t51529 * t4142;
    let t54429 = t13953 * t14648;
    let t54435 = F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t51877;
    let t54463 = t13972 * t14684;
    (t54397, t54401, t54427, t54429, t54435, t54463)
}
