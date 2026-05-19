//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 535/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk535<F: Float>(t1233: F, t3055: F, t1240: F, t864: F, t1035: F, t322: F, t407: F, t441: F, t1160: F, t180: F, t879: F, t1236: F, t930: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3057 = F::cast_from(0.39512695097613069591e1_f64) * t3055 * t1233;
    let t3058 = t1240 * t864;
    let t3059 = t1035 * t3058;
    let t3062 = t441 * t322 * t407;
    let t3063 = t1160 * t3062;
    let t3065 = t180 * t879;
    let t3066 = t3065 * t407;
    let t3067 = t1160 * t3066;
    let t3070 = t1236 * t930;
    (t3057, t3058, t3059, t3062, t3063, t3065, t3066, t3067, t3070)
}
