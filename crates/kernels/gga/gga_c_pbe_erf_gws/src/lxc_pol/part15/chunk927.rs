//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 927/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk927<F: Float>(t2410: F, t8787: F, t9283: F, t3317: F, t840: F, t1120: F, t4442: F, t8713: F, t352: F, t6126: F) -> (F, F, F, F, F, F, F) {
    let t9284 = t8787 * t2410;
    let t9285 = t9283 * t9284;
    let t9289 = 7.0 / 144.0 * t840 * t3317;
    let t9290 = t4442 * t1120;
    let t9292 = t8713 * t2410;
    let t9293 = t9283 * t9292;
    let t9296 = t352 * t6126;
    (t9284, t9285, t9289, t9290, t9292, t9293, t9296)
}
