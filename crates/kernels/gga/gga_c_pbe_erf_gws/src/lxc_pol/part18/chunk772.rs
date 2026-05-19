//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 772/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk772<F: Float>(t1764: F, t187: F, t22: F, t1878: F, t586: F, t1778: F, t633: F, t198: F, t2735: F, t185: F, t5081: F, t1903: F, t720: F) -> (F, F, F, F, F, F) {
    let t5292 = F::new(1.0) / t187 / t1764;
    let t5293 = t22 * t5292;
    let t5312 = t1878 * t586;
    let t5355 = t633 * t1778;
    let t5357 = t2735 * t198;
    let t5359 = F::new(16.0) / F::new(405.0) * t185 * t5357;
    let t5360 = F::cast_from(0.58774074074074074074e-2_f64) * t5081;
    let t5384 = F::new(2.0) / F::new(9.0) * t720 * t1903;
    (t5293, t5312, t5355, t5359, t5360, t5384)
}
