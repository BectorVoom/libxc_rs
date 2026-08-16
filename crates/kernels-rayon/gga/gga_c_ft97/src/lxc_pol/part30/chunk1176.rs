//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1176/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1176(t1466: f64, t36104: f64, t681: f64, t36096: f64, t28960: f64, t7581: f64, t111668: f64, t142918: f64, t142925: f64, t153372: f64, t193: f64, t28966: f64, t28972: f64, t29017: f64, t29033: f64, t317: f64, t33983: f64, t4309: f64, t6222: f64, t7612: f64, t798: f64) -> f64 {
    let t154911 = t1466 * t681 * t36104;
    let t154914 = t1466 * t681 * t36096;
    let t154941 = t7581 * t28960;
    let t154945 = -t154911 / 18.0_f64 - t154914 / 9.0_f64 - t1466 * t193 * t33983 * t29033 / 3.0_f64 - t1466 * t193 * t33983 * t28966 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t1466 * t193 * t6222 * t111668 + t1466 * t193 * t7612 * t4309 / 6.0_f64 + t7581 * t29017 / 6.0_f64 - t142918 / 18.0_f64 - t142925 / 9.0_f64 + t1466 * t193 * t798 * t153372 * t317 / 6.0_f64 - t154941 / 18.0_f64 - t7581 * t28972 / 3.0_f64;
    t154945
}
