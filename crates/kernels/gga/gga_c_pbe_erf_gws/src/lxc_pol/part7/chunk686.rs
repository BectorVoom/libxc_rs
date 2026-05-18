//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 686/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk686<F: Float>(t5513: F, t1678: F, t577: F, t184: F, t199: F, t266: F, t331: F, t265: F, t1640: F, t649: F, t1692: F, t661: F) -> (F, F, F, F, F, F, F, F) {
    let t5514 = F::new(4.0) / F::new(45.0) * t5513;
    let t5515 = t1678 * t577;
    let t5516 = t5515 * t184;
    let t5518 = F::new(4.0) / F::new(5.0) * t5516 * t199;
    let t5519 = t266 * t331;
    let t5521 = F::new(8.0) / F::new(405.0) * t265 * t5519;
    let t5522 = t1640 * t649;
    let t5523 = t1692 * t661;
    (t5514, t5515, t5516, t5518, t5519, t5521, t5522, t5523)
}
