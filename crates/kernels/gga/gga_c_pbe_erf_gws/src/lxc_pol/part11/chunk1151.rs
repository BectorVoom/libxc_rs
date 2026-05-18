//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1151/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1151<F: Float>(t25354: F, t1024: F, t40790: F, t42109: F, t42131: F, t3399: F, t3456: F, t16575: F, t16577: F, t16579: F) -> (F, F, F, F, F, F) {
    let t48313 = F::new(64.0) / F::new(405.0) * t25354;
    let t48315 = F::new(16.0) / F::new(15.0) * t40790 * t1024;
    let t48316 = F::new(64.0) / F::new(45.0) * t42109;
    let t48318 = F::new(32.0) / F::new(15.0) * t42131;
    let t48320 = F::new(16.0) / F::new(5.0) * t3399 * t3456;
    let t48321 = -t16575 - t16577 - t16579;
    (t48313, t48315, t48316, t48318, t48320, t48321)
}
