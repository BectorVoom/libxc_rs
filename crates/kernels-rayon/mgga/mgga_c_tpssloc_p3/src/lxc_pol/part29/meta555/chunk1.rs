//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1957/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1957(t1222: f64, t8049: f64, t5017: f64, t7337: f64, t1207: f64, t1218: f64, t2136: f64, t24675: f64, t24681: f64, t24690: f64, t24704: f64, t27578: f64, t27580: f64, t27586: f64, t27589: f64, t488: f64, t4974: f64, t5014: f64, t5030: f64, t7339: f64, t7345: f64) -> (f64, f64, f64) {
    let t27592 = t8049 * t1222;
    let t27598 = t7337 * t5017;
    let t27599 = t1207 * t27598;
    let t27602 = t24675 / 2304.0_f64 - t24681 + t27578 / 2304.0_f64 + 0.80745512188280781712e-3_f64 * t27580 * t2136 - t7345 * t4974 / 1152.0_f64 - t24690 / 864.0_f64 - t24704 + t27586 * t488 / 1536.0_f64 - t27589 * t488 / 288.0_f64 - t27592 / 432.0_f64 - t7345 * t5030 / 2304.0_f64 + t7339 * t5014 / 1536.0_f64 - t27599 * t1218 / 288.0_f64;
    (t27598, t27599, t27602)
}
