//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 780/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk780(t2281: f64, t6416: f64, t2100: f64, t369: f64, t814: f64, t931: f64, t2298: f64, t322: f64, t339: f64, t6385: f64, t2074: f64, t871: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6417 = t6416 * t2281;
    let t6421 = t2100 * t369;
    let t6424 = t814 * t931;
    let t6429 = t322 * t2298;
    let t6430 = t339 * t6385;
    let t6433 = t871 * t2074;
    (t6417, t6421, t6424, t6429, t6430, t6433)
}
