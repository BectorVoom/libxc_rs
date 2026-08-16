//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 939/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk939(t510: f64, t967: f64, t5651: f64, t1083: f64, t1473: f64, t525: f64, t8108: f64, t1503: f64, t987: f64, t1477: f64, t991: f64, t551: f64) -> (f64, f64, f64, f64, f64) {
    let t8292 = t967 * t510;
    let t8293 = t5651 * t8292;
    let t8296 = t1473 * t1083;
    let t8302 = t525 * t8108;
    let t8305 = t1503 * t987;
    let t8308 = t1477 * t991;
    let t8309 = t8308 * t551;
    (t8293, t8296, t8302, t8305, t8309)
}
