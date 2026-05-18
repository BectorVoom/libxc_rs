//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1047/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1047<F: Float>(t18969: F, t408: F, t4259: F, t88: F, t18699: F, t85: F, t414: F, t4743: F, t428: F, t4358: F, t1336: F, t1423: F) -> (F, F, F, F, F, F) {
    let t18970 = F::new(72.0) * t18969;
    let t18972 = t408 * t4259 * t88;
    let t18973 = F::new(1920.0) * t18972;
    let t18975 = F::new(0.19751789702565206229e-1) * t18699 * t85;
    let t18977 = F::new(16.0) * t414 * t4743;
    let t18978 = t4358 * t428;
    let t18979 = F::new(96.0) * t18978;
    let t18980 = t1336 * t1423;
    (t18970, t18973, t18975, t18977, t18979, t18980)
}
