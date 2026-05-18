//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 664/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk664<F: Float>(t5284: F, t587: F, t4360: F, t591: F, t590: F, t1764: F, t187: F, t22: F, t197: F, t4951: F, t4352: F, t1802: F, t1804: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5285 = t587 * t5284;
    let t5286 = F::new(8.0) / F::new(27.0) * t5285;
    let t5287 = t591 * t4360;
    let t5288 = t590 * t5287;
    let t5290 = F::new(4.0) / F::new(45.0) * t587 * t5288;
    let t5292 = F::new(1.0) / t187 / t1764;
    let t5293 = t22 * t5292;
    let t5294 = t197 * t4951;
    let t5295 = t5294 * t4352;
    let t5296 = t5293 * t5295;
    let t5298 = F::new(32.0) / F::new(81.0) * t587 * t5296;
    let t5299 = t1802 * t1804;
    (t5286, t5287, t5288, t5290, t5292, t5293, t5294, t5295, t5296, t5298, t5299)
}
