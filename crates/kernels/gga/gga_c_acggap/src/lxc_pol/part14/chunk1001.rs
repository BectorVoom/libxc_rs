//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1001/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1001<F: Float>(t35392: F, t1446: F, t7605: F, t1441: F, t1456: F, t1462: F, t1998: F, t4720: F, t1298: F, t7380: F, t7381: F, t1524: F, t1983: F, t2095: F) -> (F, F, F, F, F, F, F, F) {
    let t35393 = F::new(0.24009450146119052704e-1) * t35392;
    let t35394 = t7605 * t1446;
    let t35395 = F::new(0.68598428988911579156e-2) * t35394;
    let t35396 = t7605 * t1441;
    let t35397 = F::new(0.68598428988911579156e-2) * t35396;
    let t35398 = t7605 * t1456;
    let t35399 = F::new(0.34299214494455789578e-2) * t35398;
    let t35400 = t7605 * t1462;
    let t35403 = t1998 * t4720;
    let t35404 = F::new(0.17149607247227894789e-2) * t35403;
    let t35407 = t7380 * t7381 * t1298;
    let t35408 = t35407 / F::new(32.0);
    let t35410 = t2095 * t1983 * t1524;
    (t35393, t35395, t35397, t35399, t35400, t35404, t35408, t35410)
}
