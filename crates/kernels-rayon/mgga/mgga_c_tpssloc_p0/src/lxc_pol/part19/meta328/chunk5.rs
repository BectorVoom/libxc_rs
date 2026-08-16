//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1174/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1174(t1369: f64, t40292: f64, t12361: f64, t3866: f64, t12336: f64, t12379: f64, t12392: f64, t12397: f64, t12404: f64, t12429: f64, t1341: f64, t1343: f64, t1363: f64, t1367: f64, t3778: f64, t3858: f64, t3876: f64, t39892: f64, t40206: f64, t40271: f64, t40282: f64, t40285: f64, t40287: f64, t820: f64) -> f64 {
    let t40293 = t40292 * t1369;
    let t40295 = t3866 * t12361;
    let t40303 = 7.0_f64 / 1152.0_f64 * t40206 - t3778 * t12392 / 768.0_f64 - t1341 * t1343 * t820 * t40271 / 3072.0_f64 - t3778 * t12379 / 768.0_f64 - t12397 * t3858 / 512.0_f64 + 119.0_f64 / 288.0_f64 * t40282 + 7.0_f64 / 96.0_f64 * t40285 - t40287 * t1369 / 192.0_f64 - t12336 * t3876 / 128.0_f64 - 119.0_f64 / 288.0_f64 * t40293 + 7.0_f64 / 288.0_f64 * t40295 - t1363 * t1367 * t820 * t39892 / 768.0_f64 + t12429 * t12404 / 64.0_f64;
    t40303
}
