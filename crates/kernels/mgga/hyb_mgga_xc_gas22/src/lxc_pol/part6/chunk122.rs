//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 122/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk122<F: Float>(t307: F, t328: F, t309: F, t101: F, t123: F, t296: F, t299: F, t304: F, t308: F, t310: F, t315: F, t316: F, t320: F, t324: F, t325: F) -> (F, F, F, F, F) {
    let t329 = t307 * t307;
    let t330 = t328 * t329;
    let t331 = t309 * t309;
    let t332 = F::new(1.0) / t331;
    let t333 = t330 * t332;
    let t338 = F::cast_from(0.46914023462026644e0_f64) * t296 * t101 * t299 + t304 * t123 + t308 * t310 + F::cast_from(0.10661445329398457901e-1_f64) * t316 * t325 + F::cast_from(0.10661445329398457901e-1_f64) * t333 * t315 * t320 * t324;
    (t330, t331, t332, t333, t338)
}
