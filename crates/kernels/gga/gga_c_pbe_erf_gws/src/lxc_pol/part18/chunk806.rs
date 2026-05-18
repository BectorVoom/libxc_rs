//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 806/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk806<F: Float>(t338: F, t348: F, t6594: F, t2123: F, t6183: F, t326: F, t6469: F, t2200: F, t855: F, t859: F, t854: F, t899: F, t912: F, t923: F) -> (F, F, F, F, F, F) {
    let t6597 = F::new(455.0) / F::new(1296.0) * t348 * t6594 * t338;
    let t6605 = t6183 * t2123;
    let t6608 = t326 * t6469;
    let t6616 = t855 * t2200 * t859;
    let t6617 = t854 * t6616;
    let t6627 = t899 * t912 * t923;
    (t6597, t6605, t6608, t6616, t6617, t6627)
}
