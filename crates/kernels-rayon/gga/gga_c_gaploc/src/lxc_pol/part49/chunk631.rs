//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 631/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk631(t10455: f64, t1572: f64, t3384: f64, t4950: f64, t10414: f64, t10415: f64, t10416: f64, t10418: f64, t10423: f64, t10426: f64, t10428: f64, t10433: f64, t10437: f64, t10441: f64, t10443: f64, t10446: f64, t10450: f64, t10452: f64, t1424: f64, t1450: f64) -> (f64, f64, f64) {
    let t10457 = 0.47667319935800568892e0_f64 * t1572 * t10455;
    let t10459 = 0.71500979903700853338e0_f64 * t4950 * t3384;
    let t10460 = t10414 - t10415 + t10416 - 0.39722766613167140743e-1_f64 * t10418 * t1424 - t10423 + t10426 + t10428 + t10433 - t10437 + t10441 - t10443 - t10446 - t10450 - 0.23005755572352449806e1_f64 * t1450 * t10452 + t10457 + t10459;
    (t10457, t10459, t10460)
}
