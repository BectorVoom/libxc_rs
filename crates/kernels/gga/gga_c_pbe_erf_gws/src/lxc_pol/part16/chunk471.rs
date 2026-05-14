//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 471/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk471<F: Float>(t43: F, t50: F, t310: F, t311: F, t1: F, t305: F, t152: F, t6: F, t279: F, t837: F, t1524: F, t1526: F, t1529: F, t1531: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t2057 = 1.0 / t311 / t310;
    let t2059 = t305 * t2057 * t1;
    let t2060 = t152 * t6;
    let t2062 = t2060 * t837 * t279;
    let t2063 = t2059 * t2062;
    let t2064 = 0.63272429661648472106e0 * t2063;
    let t2068 = piecewise3(t44, 0.0, -2.0 / 9.0 * t1524 + 2.0 / 3.0 * t1526);
    let t2072 = piecewise3(t51, 0.0, -2.0 / 9.0 * t1529 + 2.0 / 3.0 * t1531);
    let t2074 = t2068 / 2.0 + t2072 / 2.0;
    (t2057, t2059, t2060, t2062, t2064, t2074)
}
