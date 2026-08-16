//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2660/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2660(t12620: f64, t12630: f64, t12709: f64, t1427: f64, t1434: f64, t19326: f64, t19405: f64, t19441: f64, t2244: f64, t2245: f64, t2283: f64, t2284: f64, t2304: f64, t33: f64, t3998: f64, t4018: f64, t5392: f64, t5393: f64, t5427: f64, t5442: f64, t55723: f64, t55751: f64, t55801: f64, t55867: f64, t629: f64, t642: f64, t65: f64, t66: f64, t72: f64, t80: f64) -> f64 {
    let t55875 = t12709 * t1434 / 12.0_f64 + t3998 * t4018 / 6.0_f64 + t19405 * t642 / 12.0_f64 + t33 * (t55751 + t55801) * t80 / 24.0_f64 - t55723 * t65 * t80 / 6.0_f64 - t2244 * t5427 * t80 / 12.0_f64 - t5392 * t2283 * t80 / 12.0_f64 - t19326 * t642 / 6.0_f64 - t5393 * t2304 / 12.0_f64 + t1427 * t12620 / 12.0_f64 + t2284 * t5442 / 24.0_f64 + t629 * t19441 / 12.0_f64 + t66 * t72 * t55867 / 24.0_f64 - t12630 * t1434 / 6.0_f64 - t2245 * t5442 / 12.0_f64;
    t55875
}
