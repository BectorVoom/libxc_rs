//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1188/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1188(t3698: f64, t3700: f64, t1297: f64, t1390: f64, t16490: f64, t193: f64, t3719: f64, t39852: f64, t39854: f64, t39856: f64, t39858: f64, t39892: f64, t39932: f64, t40222: f64, t40224: f64, t40226: f64, t40228: f64, t40230: f64, t40232: f64, t40234: f64, t40603: f64, t533: f64) -> f64 {
    let t40608 = t3698 * t3698;
    let t40610 = t3700 * t3700;
    let t40611 = 1.0_f64 / t40610;
    let t40615 = 36.0_f64 * t193 * t16490 * t3719 - t39852 + t39854 + t39856 - t39858 + 3.0_f64 * t193 * t1297 * t39892 + t193 * t533 * (t39932 + t40603) * t1390 + t40222 + t40224 + t40226 + t40228 - 6.0_f64 * t193 * t533 * t40608 * t40611 - t40230 + t40232 - t40234;
    t40615
}
