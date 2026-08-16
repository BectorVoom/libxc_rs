//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 908/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk908<F: Float>(t31253: F, t409: F, t2029: F, t7599: F, t2032: F, t2059: F, t2062: F, t167: F, t7309: F, t7483: F, t7310: F, t7487: F) -> (F, F, F, F, F, F, F, F) {
    let t31254 = t31253 * t409;
    let t31258 = t7599 * t2029;
    let t31259 = t31258 * t2032;
    let t31261 = t7599 * t2059;
    let t31262 = t31261 * t2062;
    let t31276 = t7309 * t167;
    let t31277 = t31276 * t7483;
    let t31279 = t7310 * t7487;
    (t31254, t31258, t31259, t31261, t31262, t31276, t31277, t31279)
}
