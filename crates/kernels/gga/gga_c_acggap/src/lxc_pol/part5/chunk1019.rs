//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1019/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1019<F: Float>(t406: F, t495: F, t1454: F, t322: F, t13287: F, t13293: F, t525: F, t13298: F, t176: F, t5730: F, t8401: F, t13299: F, t17173: F, t5605: F, t8790: F, t13286: F, t1530: F, t15384: F, t15386: F, t15389: F, t15392: F, t15396: F, t17179: F, t17185: F, t301: F, t372: F, t4263: F, t5615: F, t5621: F, t5715: F) -> (F, F, F) {
    let t20305 = t495 * t406;
    let t20311 = t1454 * t322;
    let t20314 = t13293 * t13287 * t525 * t20311;
    let t20323 = t13298 * t176 * t8401 * t5730;
    let t20328 = t17173 * t13299 * t8790 * t5605 * t322;
    let t20346 = -0.68598428988911579156e-2 * t13286 * t13299 * t8401 * t5715 + 0.34299214494455789578e-1 * t1530 * t15392 * t176 * t8790 * t20305 * t301 - 0.34299214494455789578e-2 * t20314 + 0.13719685797782315831e-1 * t17185 * t13299 * t8790 * t5605 * t301 - 0.34299214494455789578e-2 * t20323 + 0.34299214494455789578e-2 * t20328 - 0.68598428988911579156e-2 * t17179 * t13287 * t8790 * t5615 * t372 - 0.34299214494455789578e-2 * t15384 + 0.51448821741683684366e-2 * t15389 + 0.20579528696673473746e-1 * t13286 * t15386 * t525 * t4263 + 0.17149607247227894789e-1 * t15396 - 0.68598428988911579156e-2 * t13286 * t13299 * t8401 * t5621;
    (t20305, t20311, t20346)
}
