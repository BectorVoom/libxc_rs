//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 768/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk768(t2171: f64, t2345: f64, t6282: f64, t2157: f64, t810: f64, t2113: f64, t2257: f64, t2255: f64, t745: f64, t874: f64, t343: f64, t851: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6284 = t2345 * t6282 * t2171;
    let t6287 = t2157 * t810;
    let t6289 = t2345 * t6282 * t6287;
    let t6292 = t2113 * t2257;
    let t6293 = t2255 * t6292;
    let t6296 = t745 * t874;
    let t6297 = t6296 * t343;
    let t6298 = t851 * t6297;
    (t6284, t6287, t6289, t6293, t6297, t6298)
}
