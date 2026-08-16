//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 471/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk471(t19: f64, t793: f64, t796: f64, t801: f64, t116: f64, t299: f64, t799: f64, t798: f64, t1267: f64, t1271: f64, t1394: f64, t1398: f64, t1401: f64, t1424: f64, t1431: f64, t1433: f64, t1436: f64, t1442: f64, t1446: f64, t2064: f64) -> (f64, f64, f64) {
    let t2092 = t793 * t796 * t19;
    let t2093 = t2092 * t801;
    let t2094 = 0.82152657680133333336e0_f64 * t2093;
    let t2096 = t799 * t299 * t116;
    let t2097 = t798 * t2096;
    let t2098 = 0.6846054806677777778e0_f64 * t2097;
    let t2099 = -t2064 + t1442 + t1424 - t1431 + t1433 - t1271 - t1436 + t1446 - t2094 - t1267 + t2098 - t1394 - t1398 - t1401;
    (t2092, t2096, t2099)
}
