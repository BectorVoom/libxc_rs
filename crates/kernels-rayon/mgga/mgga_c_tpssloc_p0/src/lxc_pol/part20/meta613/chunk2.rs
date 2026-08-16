//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2203/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2203(t111: f64, t12723: f64, t2363: f64, t649: f64, t89: f64, t9416: f64, t11968: f64, t12492: f64, t12557: f64, t1266: f64, t12725: f64, t12734: f64, t12813: f64, t12816: f64, t12823: f64, t12835: f64, t1393: f64, t1458: f64, t1459: f64, t1778: f64, t1849: f64, t19456: f64, t2314: f64, t2364: f64, t4037: f64, t652: f64, t672: f64, t9419: f64) -> (f64, f64, f64) {
    let t45632 = t12723 * t111;
    let t45637 = t649 * t2363;
    let t45640 = t89 * t9416;
    let t45648 = -2.0_f64 * t11968 * t1458 * t652 - 6.0_f64 * t1266 * t12813 * t652 + t12492 * t1778 - 6.0_f64 * t12557 * t2314 - 6.0_f64 * t12725 * t2364 - 12.0_f64 * t12734 * t4037 + 3.0_f64 * t12816 * t1393 - 6.0_f64 * t12823 * t4037 - 6.0_f64 * t12835 * t2314 - 6.0_f64 * t1459 * t45637 - 2.0_f64 * t1459 * t45640 + t1849 * t9419 - 6.0_f64 * t19456 * t2364 - 6.0_f64 * t45632 * t672;
    (t45632, t45637, t45648)
}
