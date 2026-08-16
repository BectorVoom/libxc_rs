//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 122/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk122(t307: f64, t328: f64, t309: f64, t101: f64, t123: f64, t296: f64, t299: f64, t304: f64, t308: f64, t310: f64, t315: f64, t316: f64, t320: f64, t324: f64, t325: f64) -> (f64, f64, f64, f64, f64) {
    let t329 = t307 * t307;
    let t330 = t328 * t329;
    let t331 = t309 * t309;
    let t332 = 1.0_f64 / t331;
    let t333 = t330 * t332;
    let t338 = 0.46914023462026644e0_f64 * t296 * t101 * t299 + t304 * t123 + t308 * t310 + 0.10661445329398457901e-1_f64 * t316 * t325 + 0.10661445329398457901e-1_f64 * t333 * t315 * t320 * t324;
    (t330, t331, t332, t333, t338)
}
