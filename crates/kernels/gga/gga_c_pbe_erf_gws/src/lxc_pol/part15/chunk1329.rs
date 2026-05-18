//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1329/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1329<F: Float>(t14547: F, t20842: F, t27363: F, t51274: F, t8906: F, t14046: F, t3172: F, t14565: F, t346: F, t838: F, t859: F, t27823: F, t3139: F, t4028: F) -> (F, F, F, F, F) {
    let t54391 = t14547 * t20842 * t27363;
    let t54394 = t51274 * t8906;
    let t54397 = t14046 * t3172;
    let t54398 = F::new(7.0) / F::new(144.0) * t54397;
    let t54401 = t14565 * t346 * t838 * t859;
    let t54402 = F::new(7.0) / F::new(144.0) * t54401;
    let t54404 = t4028 * t3139 * t27823;
    (t54391, t54394, t54398, t54402, t54404)
}
