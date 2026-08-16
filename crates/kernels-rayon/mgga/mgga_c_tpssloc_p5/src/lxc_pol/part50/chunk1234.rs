//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1234/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1234(t119867: f64, t119869: f64, t119871: f64, t119874: f64, t119875: f64, t119877: f64, t119996: f64, t120002: f64, t120003: f64, t120005: f64, t120008: f64, t1266: f64, t1442: f64, t22461: f64, t26103: f64, t30989: f64, t32679: f64, t33124: f64, t4026: f64, t510: f64, t7472: f64, t8329: f64, t8439: f64) -> f64 {
    let t120015 = -t119996 * t510 - t1266 * t33124 - t1442 * t30989 - 4.0_f64 * t22461 * t7472 - 4.0_f64 * t26103 * t7472 - t4026 * t8439 - t119867 - 4.0_f64 * t119869 - 4.0_f64 * t119871 - t119874 + 2.0_f64 * t119875 + t119877 + t120002 - 2.0_f64 * t120003 - 2.0_f64 * t120005 - t120008 - t32679 - t8329;
    t120015
}
