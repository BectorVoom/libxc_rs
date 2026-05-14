//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 854/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk854<F: Float>(t17438: F, t5399: F, t9: F, t5402: F, t639: F, t17037: F, t219: F, t16973: F, t5400: F, t1652: F, t5406: F, t1898: F, t17420: F, t17425: F, t17430: F, t17432: F, t17434: F, t17436: F) -> (F, F, F, F, F, F) {
    let t17439 = 32.0 / 135.0 * t17438;
    let t17440 = t9 * t5399;
    let t17442 = t639 * t17440 * t5402;
    let t17443 = 256.0 / 243.0 * t17442;
    let t17444 = t219 * t17037;
    let t17448 = 128.0 / 27.0 * t639 * t5400 * t17444 * t16973;
    let t17449 = t5406 * t1652;
    let t17450 = 32.0 / 45.0 * t17449;
    let t17452 = 16.0 / 15.0 * t5406 * t1898;
    let t17453 = -t17420 - t17425 - t17430 + t17432 + t17434 + t17436 + t17439 + t17443 - t17448 + t17450 - t17452;
    (t17439, t17443, t17448, t17450, t17452, t17453)
}
