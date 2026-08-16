//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2317/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2317(t27604: f64, t3523: f64, t1232: f64, t1748: f64, t2132: f64, t2136: f64, t3587: f64, t86129: f64, t86228: f64, t86248: f64, t88391: f64, t95446: f64, t95450: f64, t95452: f64, t95456: f64, t95459: f64, t95463: f64) -> f64 {
    let t95465 = t27604 * t3523 / 324.0_f64;
    let t95469 = t95446 - 0.10093189023535097714e-3_f64 * t2132 * t88391 * t2136 + t95450 / 162.0_f64 + t95452 * t1232 / 216.0_f64 - t95456 + t86228 / 2304.0_f64 - t95459 - 5.0_f64 / 1296.0_f64 * t27604 * t3587 - t95463 + t95465 - t86129 * t1748 / 2304.0_f64 + 0.10093189023535097714e-3_f64 * t86248;
    t95469
}
