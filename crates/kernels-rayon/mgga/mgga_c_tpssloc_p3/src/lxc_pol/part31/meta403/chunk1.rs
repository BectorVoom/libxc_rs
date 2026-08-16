//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1479/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1479(t19390: f64, t607: f64, t3966: f64, t3990: f64, t2274: f64, t5398: f64, t16558: f64, t55: f64, t1420: f64, t19369: f64, t19372: f64, t19378: f64, t19381: f64, t39: f64, t3991: f64, t3994: f64, t51: f64, t5408: f64, t5411: f64, t5416: f64, t615: f64, t621: f64, t9311: f64) -> f64 {
    let t19391 = t19390 * t607;
    let t19394 = t3990 * t3966;
    let t19397 = t2274 * t5398;
    let t19398 = t19397 * t607;
    let t19401 = t55 * t16558;
    let t19404 = -20.0_f64 / 27.0_f64 * t615 * t5408 - 5.0_f64 / 108.0_f64 * t39 * t19369 + 5.0_f64 / 9.0_f64 * t39 * t19372 - 20.0_f64 / 9.0_f64 * t615 * t5411 + 5.0_f64 / 18.0_f64 * t39 * t19378 + 5.0_f64 / 6.0_f64 * t39 * t19381 - 220.0_f64 / 27.0_f64 * t5416 * t621 - 40.0_f64 / 27.0_f64 * t1420 * t3991 + 40.0_f64 / 9.0_f64 * t1420 * t3994 + 5.0_f64 / 108.0_f64 * t51 * t19391 + 5.0_f64 / 9.0_f64 * t51 * t19394 + 5.0_f64 / 18.0_f64 * t51 * t19398 - 5.0_f64 / 6.0_f64 * t51 * t19401 + t9311;
    t19404
}
