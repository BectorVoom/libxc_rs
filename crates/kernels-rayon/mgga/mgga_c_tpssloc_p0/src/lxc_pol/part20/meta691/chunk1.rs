//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2624/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2624(t1213: f64, t15525: f64, t248: f64, t3570: f64, t11813: f64, t5018: f64, t15749: f64, t3577: f64, t45124: f64, t11734: f64, t1214: f64, t1218: f64, t15531: f64, t15553: f64, t3494: f64, t3515: f64, t3518: f64, t4582: f64, t475: f64, t52458: f64, t53378: f64, t53387: f64, t53389: f64, t53397: f64, t53399: f64) -> f64 {
    let t53404 = t1213 * t248 * t3570 * t15525;
    let t53406 = t11813 * t5018;
    let t53410 = t3577 * t45124 * t15749;
    let t53412 = -t53378 / 768.0_f64 - t11734 * t15531 / 1024.0_f64 - t3515 * t4582 * t15553 * t3494 / 1024.0_f64 - t53387 / 72.0_f64 - t53389 / 288.0_f64 + t1213 * t248 * t1214 * t52458 * t475 / 3072.0_f64 + t53397 / 1536.0_f64 - t53399 * t3518 / 1024.0_f64 + t53404 / 1536.0_f64 - t53406 * t1218 / 192.0_f64 + 5.0_f64 / 3456.0_f64 * t53410;
    t53412
}
