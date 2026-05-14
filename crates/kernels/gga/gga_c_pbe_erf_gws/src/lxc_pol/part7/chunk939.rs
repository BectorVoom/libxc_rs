//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 939/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk939<F: Float>(t1326: F, t1422: F, t40: F, t1322: F, t18639: F, t470: F, t4734: F, t1327: F, t1336: F, t408: F, t4259: F, t88: F, t18699: F, t85: F, t414: F, t4743: F) -> (F, F, F, F, F, F) {
    let t18963 = t40 * t1422 * t1326;
    let t18964 = 6.0 * t18963;
    let t18968 = 0.6233672123775310788e3 * t470 * t4734 * t18639 * t1322;
    let t18969 = t1336 * t1327;
    let t18970 = 72.0 * t18969;
    let t18972 = t408 * t4259 * t88;
    let t18973 = 1920.0 * t18972;
    let t18975 = 0.19751789702565206229e-1 * t18699 * t85;
    let t18977 = 16.0 * t414 * t4743;
    (t18964, t18968, t18970, t18973, t18975, t18977)
}
