//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1040/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1040<F: Float>(t11651: F, t9665: F, t3257: F, t3803: F, t6355: F, t326: F, t6469: F, t820: F, t339: F, t3802: F, t6472: F, t860: F) -> (F, F, F, F, F) {
    let t11652 = t9665 * t11651;
    let t11656 = t3257 * t3803 * t6355;
    let t11660 = t326 * t6469 * t820;
    let t11661 = t3802 * t339;
    let t11662 = t6472 * t11661;
    let t11663 = t11660 * t11662;
    let t11665 = t11663 * t860 / F::cast_from(96.0_f64);
    (t11652, t11656, t11660, t11661, t11665)
}
