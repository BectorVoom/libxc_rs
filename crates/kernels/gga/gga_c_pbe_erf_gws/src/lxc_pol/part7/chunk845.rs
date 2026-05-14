//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 845/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk845<F: Float>(t5357: F, t579: F, t5372: F, t586: F, t645: F, t1651: F, t5509: F, t587: F, t1648: F, t5413: F, t5284: F, t17285: F, t17287: F, t17291: F, t17293: F, t17297: F, t17300: F) -> (F, F, F, F, F, F) {
    let t17301 = t579 * t5357;
    let t17302 = 64.0 / 405.0 * t17301;
    let t17303 = t5372 * t586;
    let t17305 = 16.0 / 45.0 * t17303 * t645;
    let t17307 = t587 * t1651 * t5509;
    let t17308 = 64.0 / 45.0 * t17307;
    let t17309 = t1648 * t5413;
    let t17310 = 32.0 / 45.0 * t17309;
    let t17311 = t1648 * t5284;
    let t17312 = 32.0 / 27.0 * t17311;
    let t17313 = t17285 + t17287 + t17291 - t17293 - t17297 + t17300 + t17302 + t17305 + t17308 + t17310 + t17312;
    (t17302, t17305, t17308, t17310, t17312, t17313)
}
