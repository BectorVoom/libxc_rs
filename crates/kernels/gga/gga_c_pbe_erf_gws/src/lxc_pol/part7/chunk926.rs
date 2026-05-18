//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 926/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk926<F: Float>(t1666: F, t5406: F, t5400: F, t649: F, t5064: F, t639: F, t661: F, t17014: F, t7853: F, t5378: F, t586: F, t1824: F) -> (F, F, F, F, F) {
    let t17330 = F::new(8.0) / F::new(9.0) * t5406 * t1666;
    let t17331 = t5400 * t649;
    let t17335 = F::new(128.0) / F::new(81.0) * t639 * t17331 * t5064 * t661;
    let t17338 = F::new(64.0) / F::new(27.0) * t639 * t7853 * t17014;
    let t17339 = t5378 * t586;
    let t17341 = F::new(32.0) / F::new(15.0) * t17339 * t1824;
    (t17330, t17335, t17338, t17339, t17341)
}
