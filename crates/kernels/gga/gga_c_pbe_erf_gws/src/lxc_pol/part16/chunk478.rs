//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 478/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk478<F: Float>(t2092: F, t801: F, t116: F, t299: F, t799: F, t798: F, t1267: F, t1271: F, t1394: F, t1398: F, t1401: F, t1424: F, t1431: F, t1433: F, t1436: F, t1442: F, t1446: F, t2064: F) -> (F, F, F, F) {
    let t2093 = t2092 * t801;
    let t2094 = 0.82152657680133333336e0 * t2093;
    let t2096 = t799 * t299 * t116;
    let t2097 = t798 * t2096;
    let t2098 = 0.6846054806677777778e0 * t2097;
    let t2099 = -t2064 + t1442 + t1424 - t1431 + t1433 - t1271 - t1436 + t1446 - t2094 - t1267 + t2098 - t1394 - t1398 - t1401;
    (t2093, t2096, t2098, t2099)
}
