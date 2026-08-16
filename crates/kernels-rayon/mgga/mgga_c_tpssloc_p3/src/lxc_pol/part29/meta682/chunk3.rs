//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2307/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2307(t15437: f64, t24728: f64, t24732: f64, t4965: f64, t7344: f64, t1232: f64, t1737: f64, t27604: f64, t27614: f64, t27617: f64, t3496: f64, t3511: f64, t3518: f64, t3527: f64, t3531: f64, t86122: f64, t86124: f64, t86126: f64, t86136: f64) -> f64 {
    let t95270 = t15437 * t24728;
    let t95273 = t15437 * t24732;
    let t95276 = t4965 * t7344;
    let t95285 = t86122 / 1152.0_f64 - t86124 / 1728.0_f64 - t86136 / 1728.0_f64 + t27604 * t3527 / 432.0_f64 + t27604 * t3531 / 216.0_f64 + t27614 * t3496 / 1536.0_f64 + t95270 * t3511 / 768.0_f64 - t95273 * t3518 / 1536.0_f64 - t95276 * t1232 / 1152.0_f64 - t27617 * t3527 / 2304.0_f64 - t27617 * t3531 / 1152.0_f64 + t86126 * t1737 / 1536.0_f64;
    t95285
}
