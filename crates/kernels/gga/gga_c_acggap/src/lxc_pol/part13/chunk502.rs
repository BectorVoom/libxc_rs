//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 502/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk502<F: Float>(t3044: F, t452: F, t1035: F, t1252: F, t377: F, t309: F, t861: F, t150: F, t1233: F, t1240: F, t864: F, t322: F, t407: F, t441: F, t1160: F, t180: F, t879: F) -> (F, F, F, F, F, F, F, F) {
    let t3045 = t452 * t3044;
    let t3047 = 0.39512695097613069591e1 * t1035 * t3045;
    let t3048 = t377 * t1252;
    let t3054 = t309 * t861;
    let t3055 = t3054 * t150;
    let t3057 = 0.39512695097613069591e1 * t3055 * t1233;
    let t3058 = t1240 * t864;
    let t3059 = t1035 * t3058;
    let t3062 = t441 * t322 * t407;
    let t3063 = t1160 * t3062;
    let t3065 = t180 * t879;
    (t3047, t3048, t3054, t3055, t3057, t3059, t3063, t3065)
}
