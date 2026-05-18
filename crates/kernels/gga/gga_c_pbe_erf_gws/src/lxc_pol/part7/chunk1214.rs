//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1214/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1214<F: Float>(t6249: F, t6542: F, t20135: F, t20137: F, t20619: F, t21065: F, t21528: F, t21534: F, t21537: F, t21540: F, t21544: F, t21563: F, t2343: F, t2345: F, t6220: F, t6282: F, t902: F, t904: F, t905: F, t914: F, t916: F, t9665: F) -> (F, F) {
    let t21564 = t6542 * t6249;
    let t21565 = F::new(7.0) / F::new(12.0) * t21564;
    let t21566 = t21528 - t21534 - F::new(7.0) / F::new(96.0) * t21537 - t21540 + t21544 - t914 * t916 * t904 * t20619 / F::new(512.0) + t2343 * t9665 * t21065 / F::new(32.0) + t902 * t905 * t20135 * t20137 / F::new(192.0) + t2343 * t2345 * t6282 * t6220 / F::new(64.0) - t21563 - t21565;
    (t21565, t21566)
}
