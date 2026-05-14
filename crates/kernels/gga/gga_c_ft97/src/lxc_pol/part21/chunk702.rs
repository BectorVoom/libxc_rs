//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 702/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk702<F: Float>(t16704: F, t16930: F, t515: F, t16150: F, t3440: F, t2210: F, t16169: F, t3434: F, t4417: F, t558: F) -> (F, F, F, F, F, F) {
    let t16931 = t16704 + t16930;
    let t16932 = t515 * t16931;
    let t16942 = t3440 * t16150;
    let t16943 = t2210 * t16942;
    let t16946 = t3434 * t16169;
    let t16947 = t2210 * t16946;
    let t16950 = t4417 * t558;
    (t16932, t16942, t16943, t16946, t16947, t16950)
}
