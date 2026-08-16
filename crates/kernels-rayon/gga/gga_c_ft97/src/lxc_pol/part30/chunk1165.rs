//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1165/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1165(t1882: f64, t36188: f64, t36142: f64, t8392: f64, t36232: f64, t36130: f64, t36250: f64, t114271: f64, t143621: f64, t143924: f64, t143953: f64, t152780: f64, t153560: f64, t15460: f64, t1901: f64, t2862: f64, t28847: f64, t29128: f64, t29129: f64, t29189: f64, t296: f64, t319: f64, t35833: f64, t4176: f64, t4181: f64, t446: f64, t44600: f64, t53797: f64, t7672: f64, t882: f64) -> (f64, f64, f64, f64) {
    let t154338 = t1882 * t36188;
    let t154357 = t8392 * t36142;
    let t154359 = t1882 * t36232;
    let t154362 = t1882 * t36130;
    let t154392 = t1882 * t36250;
    let t154394 = -4.0_f64 / 9.0_f64 * t154362 - 2.0_f64 / 9.0_f64 * t143924 + 4.0_f64 / 3.0_f64 * t446 * t2862 * t882 * t35833 + 4.0_f64 / 3.0_f64 * t446 * t2862 * t319 * t152780 + 4.0_f64 / 9.0_f64 * t53797 * t114271 * t29189 + 2.0_f64 * t1901 * t15460 * t143621 * t4176 + 8.0_f64 * t1901 * t29128 * t44600 * t7672 * t4181 - 4.0_f64 * t1901 * t29128 * t29129 * t28847 + 2.0_f64 / 3.0_f64 * t446 * t296 * t153560 - 2.0_f64 / 9.0_f64 * t154392 - t143953;
    (t154338, t154357, t154359, t154394)
}
