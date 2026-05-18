//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 630/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk630<F: Float>(t1812: F, t4913: F, t1627: F, t1817: F, t1403: F, t1764: F, t562: F, t1821: F, t1820: F, t1765: F, t610: F, t1827: F) -> (F, F, F, F, F, F, F) {
    let t4915 = F::new(16.0) / F::new(15.0) * t4913 * t1812;
    let t4917 = F::new(8.0) / F::new(15.0) * t1627 * t1817;
    let t4919 = t562 * t1764 * t1403;
    let t4920 = t1821 * t4919;
    let t4922 = F::new(16.0) / F::new(15.0) * t1820 * t4920;
    let t4923 = t1765 * t610;
    let t4924 = t1827 * t4923;
    (t4915, t4917, t4919, t4920, t4922, t4923, t4924)
}
