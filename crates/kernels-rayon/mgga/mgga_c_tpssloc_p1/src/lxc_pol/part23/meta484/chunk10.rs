//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1482/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1482(t5389: f64, t5445: f64, t1411: f64, t1426: f64, t1427: f64, t1434: f64, t19420: f64, t19430: f64, t20210: f64, t20217: f64, t20218: f64, t20219: f64, t20285: f64, t2291: f64, t2298: f64, t31: f64, t39096: f64, t39114: f64, t4007: f64, t4012: f64, t5392: f64, t5393: f64, t5398: f64, t5403: f64, t5427: f64, t5428: f64, t5442: f64, t634: f64, t638: f64, t65: f64, t66: f64, t72: f64, t75836: f64, t75847: f64, t75912: f64, t80: f64) -> (f64, f64, f64) {
    let t79579 = t5389 * t5389;
    let t79585 = t5445 * t5445;
    let t79637 = -t5392 * t5427 * t80 / 2.0_f64 - t20210 * t1434 - t5393 * t5442 / 2.0_f64 - t5403 * t5442 - t1411 * t20285 / 3.0_f64 + t5428 * t5442 / 4.0_f64 + t1427 * t20285 / 6.0_f64 + t66 * t72 * (3640.0_f64 / 81.0_f64 * t39096 * t75836 - 560.0_f64 / 9.0_f64 * t19420 * t5398 + 28.0_f64 / 3.0_f64 * t2291 * t75847 + 112.0_f64 / 9.0_f64 * t4007 * t20217 - 4.0_f64 / 3.0_f64 * t634 * t75912 + 3640.0_f64 / 81.0_f64 * t39114 * t75836 + 560.0_f64 / 9.0_f64 * t19430 * t5398 + 28.0_f64 / 3.0_f64 * t2298 * t75847 + 112.0_f64 / 9.0_f64 * t4012 * t20217 + 4.0_f64 / 3.0_f64 * t638 * t75912) / 24.0_f64 - t31 * t75912 * t65 * t80 / 12.0_f64 - t20218 * t1426 * t80 / 3.0_f64 - t20219 * t1434 / 3.0_f64;
    (t79579, t79585, t79637)
}
