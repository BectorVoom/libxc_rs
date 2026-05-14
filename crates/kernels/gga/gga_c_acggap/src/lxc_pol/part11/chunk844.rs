//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 844/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk844<F: Float>(t1459: F, t1980: F, t31024: F, t7458: F, t2117: F, t980: F, t409: F, t7712: F, t932: F, t2029: F, t7599: F, t2032: F, t2059: F, t2062: F, t167: F, t7309: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31251 = t1980 * t7458 * t1459 * t31024;
    let t31253 = t980 * t2117;
    let t31254 = t31253 * t409;
    let t31256 = t7712 * t932;
    let t31258 = t7599 * t2029;
    let t31259 = t31258 * t2032;
    let t31261 = t7599 * t2059;
    let t31262 = t31261 * t2062;
    let t31276 = t7309 * t167;
    (t31251, t31253, t31254, t31256, t31258, t31259, t31261, t31262, t31276)
}
