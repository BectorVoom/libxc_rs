//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1489/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1489(t113: f64, t12492: f64, t12507: f64, t1266: f64, t1271: f64, t12734: f64, t1393: f64, t2314: f64, t2320: f64, t2364: f64, t3652: f64, t3660: f64, t39223: f64, t39231: f64, t39235: f64, t3929: f64, t39332: f64, t39385: f64, t39480: f64, t39524: f64, t39586: f64, t39626: f64, t39847: f64, t40615: f64, t43657: f64, t45402: f64, t510: f64, t513: f64, t672: f64, t89: f64, t9347: f64, t9351: f64, t9419: f64) -> f64 {
    let t45405 = -t39223 * t510 - 24.0_f64 * t9351 * t1266 - 12.0_f64 * t2320 * t3652 - 4.0_f64 * t9347 * t1266 - 6.0_f64 * t89 * t39231 * t510 - 8.0_f64 * t39235 * t672 - 24.0_f64 * t12734 * t2364 - 24.0_f64 * t2314 * t12507 + 6.0_f64 * t3660 * t3929 + 4.0_f64 * t9419 * t1393 + t513 * (t39332 + t39385 + t39480 + t39524 + t39586 + t39626 + t39847 + t40615) + 4.0_f64 * t1271 * t12492 - t113 * (t43657 + t45402);
    t45405
}
