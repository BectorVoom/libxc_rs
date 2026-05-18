//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1038/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1038<F: Float>(t9386: F, t9388: F, t2190: F, t3219: F, t3235: F, t2345: F, t3240: F, t3140: F, t9375: F, t2494: F, t6: F, t875: F) -> (F, F, F, F, F, F) {
    let t9389 = t9386 * t9388;
    let t9393 = t3235 * t3219 * t2190;
    let t9397 = t2345 * t3240 * t2190;
    let t9401 = t3235 * t9375 * t3140;
    let t9404 = t6 * t2494;
    let t9406 = t2345 * t9404 * t875;
    (t9389, t9393, t9397, t9401, t9404, t9406)
}
