//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2214/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2214(t3082: f64, t7586: f64, t25641: f64, t82892: f64, t25638: f64, t6735: f64, t13532: f64, t13537: f64, t13797: f64, t13941: f64, t14122: f64, t14126: f64, t1920: f64, t1941: f64, t23548: f64, t23564: f64, t25679: f64, t378: f64, t4509: f64, t7574: f64, t7583: f64, t82918: f64, t82923: f64, t83016: f64, t83034: f64, t83215: f64) -> f64 {
    let t88479 = t7586 * t3082;
    let t88488 = 0.20186378047070195428e-3_f64 * t82892 * t25641;
    let t88503 = 0.20186378047070195428e-3_f64 * t25638 * t6735;
    let t88504 = t13941 * t1941 * t378 / 1536.0_f64 - t88479 / 6912.0_f64 + t1920 * t4509 * t13532 / 216.0_f64 + 7.0_f64 / 648.0_f64 * t1920 * t13797 * t13537 + t88488 - 0.10093189023535097714e-3_f64 * t83034 - 0.20186378047070195428e-3_f64 * t82918 * t7583 - 0.10093189023535097714e-3_f64 * t82923 * t7583 - 0.20186378047070195428e-3_f64 * t23564 * t25679 + t83016 * t14122 / 1152.0_f64 - t83215 * t14126 / 2304.0_f64 - 0.10093189023535097714e-3_f64 * t7574 * t23548 - t88503;
    t88504
}
