//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 837/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk837<F: Float>(t1322: F, t18639: F, t470: F, t4734: F, t1327: F, t1336: F, t408: F, t4259: F, t88: F, t414: F, t4743: F, t1332: F, t274: F, t169: F, t18411: F, t289: F) -> (F, F, F, F, F, F) {
    let t18968 = 0.6233672123775310788e3 * t470 * t4734 * t18639 * t1322;
    let t18969 = t1336 * t1327;
    let t18970 = 72.0 * t18969;
    let t18972 = t408 * t4259 * t88;
    let t18973 = 1920.0 * t18972;
    let t18977 = 16.0 * t414 * t4743;
    let t18995 = 0.6399008129061525636e1 * t1332 * t274;
    let t18998 = 0.31835665774679373271e-1 * t169 * t289 * t18411;
    (t18968, t18970, t18973, t18977, t18995, t18998)
}
