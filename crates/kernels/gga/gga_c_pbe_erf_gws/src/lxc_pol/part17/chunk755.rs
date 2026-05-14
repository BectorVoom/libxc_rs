//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 755/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk755<F: Float>(t745: F, t874: F, t343: F, t2189: F, t274: F, t2145: F, t2387: F, t2365: F, t885: F, t2149: F, t2146: F, t2157: F, t1: F, t2298: F, t253: F, t320: F, t368: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6296 = t745 * t874;
    let t6297 = t6296 * t343;
    let t6303 = t274 * t2189 * t343;
    let t6322 = t2387 * t2145;
    let t6331 = t2365 * t885;
    let t6332 = t6331 * t2149;
    let t6333 = t2146 * t6332;
    let t6360 = t2157 * t2189;
    let t6365 = t2298 * t1;
    let t6366 = t6365 * t253;
    let t6382 = 1.0 / t368 / t320;
    (t6297, t6303, t6322, t6331, t6332, t6333, t6360, t6365, t6366, t6382)
}
