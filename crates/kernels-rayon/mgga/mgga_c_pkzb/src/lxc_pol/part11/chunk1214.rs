//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1214/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1214(t12: f64, t10513: f64, t10518: f64, t1430: f64, t17361: f64, t1837: f64, t2732: f64, t28874: f64, t28877: f64, t28885: f64, t439: f64, t652: f64, t7337: f64, t7340: f64, t8729: f64, t9150: f64, zeta_threshold: f64) -> f64 {
    let t84 = t12 <= zeta_threshold;
    let t29813 = piecewise3(t84, 0.0_f64, 280.0_f64 / 81.0_f64 * t17361 * t10513 * t439 - 56.0_f64 / 9.0_f64 * t9150 * t1430 - 28.0_f64 / 9.0_f64 * t7337 * t28874 + 8.0_f64 / 3.0_f64 * t7340 * t28877 + 4.0_f64 / 3.0_f64 * t2732 * t8729 + 4.0_f64 / 9.0_f64 * t1837 * t10518 * t439 - t652 * t28885 / 3.0_f64);
    t29813
}
