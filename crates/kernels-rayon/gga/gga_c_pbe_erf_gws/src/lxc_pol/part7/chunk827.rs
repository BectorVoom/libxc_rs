//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 827/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk827(t338: f64, t353: f64, t6817: f64, t2352: f64, t938: f64, t2409: f64, t3067: f64, t2410: f64, t6781: f64, t329: f64, t369: f64, t838: f64) -> (f64, f64, f64, f64, f64) {
    let t6819 = t338 * t353 * t6817;
    let t6822 = t2352 * t938;
    let t6824 = t2409 * t3067 * t6822;
    let t6828 = t2409 * t6781 * t2410;
    let t6832 = t329 * t838 * t369;
    (t6819, t6822, t6824, t6828, t6832)
}
