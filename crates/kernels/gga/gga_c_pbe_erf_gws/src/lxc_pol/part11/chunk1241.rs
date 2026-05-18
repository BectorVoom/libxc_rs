//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1241/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1241<F: Float>(t38850: F, t45235: F, t860: F, t3793: F, t44902: F, t45228: F, t44972: F, t45240: F, t37380: F, t11478: F, t13408: F, t2168: F, t6523: F) -> (F, F, F, F, F, F, F) {
    let t49658 = t45235 * t38850 * t860 / F::new(16.0);
    let t49660 = t44902 * t3793 / F::new(32.0);
    let t49661 = F::new(7.0) / F::new(12.0) * t45228;
    let t49663 = t44972 * t3793 / F::new(16.0);
    let t49664 = F::new(7.0) / F::new(36.0) * t45240;
    let t49667 = F::new(35.0) / F::new(18.0) * t37380;
    let t49671 = F::new(3.0) / F::new(8.0) * t2168 * t6523 * t11478 * t13408;
    (t49658, t49660, t49661, t49663, t49664, t49667, t49671)
}
