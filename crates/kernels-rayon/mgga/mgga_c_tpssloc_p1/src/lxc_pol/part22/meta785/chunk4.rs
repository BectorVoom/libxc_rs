//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2707/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2707(t1427: f64, t1434: f64, t19326: f64, t19405: f64, t19441: f64, t20210: f64, t20265: f64, t20285: f64, t33: f64, t3997: f64, t3998: f64, t4018: f64, t5392: f64, t5393: f64, t5428: f64, t5442: f64, t629: f64, t642: f64, t66: f64, t72: f64, t75461: f64, t75494: f64, t75543: f64, t80: f64) -> f64 {
    let t75547 = -t5392 * t3997 * t80 / 4.0_f64 - t20210 * t642 / 4.0_f64 - t19326 * t1434 / 4.0_f64 - t5393 * t4018 / 4.0_f64 + t33 * (t75461 + t75494) * t80 / 24.0_f64 + t20265 * t642 / 24.0_f64 + t19405 * t1434 / 8.0_f64 + t5428 * t4018 / 8.0_f64 + t3998 * t5442 / 8.0_f64 + t1427 * t19441 / 8.0_f64 + t629 * t20285 / 24.0_f64 + t66 * t72 * t75543 / 24.0_f64;
    t75547
}
