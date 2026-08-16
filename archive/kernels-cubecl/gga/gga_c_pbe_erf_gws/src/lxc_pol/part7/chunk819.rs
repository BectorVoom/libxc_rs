//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 819/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk819<F: Float>(t2074: F, t938: F, t2376: F, t2409: F, t2182: F, t2383: F, t3074: F, t2112: F, t829: F, t830: F, t831: F, t2358: F, t2382: F) -> (F, F, F, F, F, F, F) {
    let t6755 = t2074 * t938;
    let t6757 = t2409 * t2376 * t6755;
    let t6760 = t2182 * t938;
    let t6762 = t2409 * t2376 * t6760;
    let t6769 = t3074 * t2383;
    let t6772 = t829 * t830 * t831 * t2112;
    let t6775 = t2382 * t2358;
    (t6755, t6757, t6760, t6762, t6769, t6772, t6775)
}
