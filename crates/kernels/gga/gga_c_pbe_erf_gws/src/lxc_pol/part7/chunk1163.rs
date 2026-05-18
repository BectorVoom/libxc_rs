//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1163/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1163<F: Float>(t6332: F, t6711: F, t6652: F, t8967: F, t2168: F, t2170: F, t6220: F, t6269: F, t20474: F, t3065: F, t858: F, t6672: F) -> (F, F, F, F) {
    let t20760 = t6711 * t6332;
    let t20761 = F::new(7.0) / F::new(12.0) * t20760;
    let t20768 = t8967 * t6652;
    let t20769 = F::new(7.0) / F::new(12.0) * t20768;
    let t20781 = t2168 * t2170 * t6269 * t6220 / F::new(8.0);
    let t20783 = t3065 * t858 * t20474;
    let t20785 = t6672 * t20783 / F::new(8.0);
    (t20761, t20769, t20781, t20785)
}
