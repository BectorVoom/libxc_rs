//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 751/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk751(t5: f64, t67: f64, t7440: f64, t1864: f64, t1433: f64, t71: f64, t1863: f64, t1860: f64, t1865: f64, t6490: f64, t7428: f64, t7432: f64, t7435: f64) -> (f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t7441 = t7440 * t67;
    let t7442 = t7441 * t1864;
    let t7445 = t71 * t1433;
    let t7446 = t1863 * t7445;
    let t7450 = piecewise3(t8, 0.0_f64, -t7428 * t1865 / 6.0_f64 + 5.0_f64 / 6.0_f64 * t6490 * t7432 + t7435 * t1865 / 3.0_f64 - t1860 * t7442 / 6.0_f64 - t1860 * t7446 / 6.0_f64);
    (t7441, t7442, t7445, t7446, t7450)
}
