//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1135/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1135(t13286: f64, t13287: f64, t13299: f64, t1530: f64, t15384: f64, t15386: f64, t15389: f64, t15392: f64, t15396: f64, t17179: f64, t17185: f64, t176: f64, t20305: f64, t20314: f64, t20323: f64, t20328: f64, t301: f64, t372: f64, t4263: f64, t525: f64, t5605: f64, t5615: f64, t5621: f64, t5715: f64, t8401: f64, t8790: f64) -> f64 {
    let t20346 = -0.68598428988911579156e-2_f64 * t13286 * t13299 * t8401 * t5715 + 0.34299214494455789578e-1_f64 * t1530 * t15392 * t176 * t8790 * t20305 * t301 - 0.34299214494455789578e-2_f64 * t20314 + 0.13719685797782315831e-1_f64 * t17185 * t13299 * t8790 * t5605 * t301 - 0.34299214494455789578e-2_f64 * t20323 + 0.34299214494455789578e-2_f64 * t20328 - 0.68598428988911579156e-2_f64 * t17179 * t13287 * t8790 * t5615 * t372 - 0.34299214494455789578e-2_f64 * t15384 + 0.51448821741683684366e-2_f64 * t15389 + 0.20579528696673473746e-1_f64 * t13286 * t15386 * t525 * t4263 + 0.17149607247227894789e-1_f64 * t15396 - 0.68598428988911579156e-2_f64 * t13286 * t13299 * t8401 * t5621;
    t20346
}
