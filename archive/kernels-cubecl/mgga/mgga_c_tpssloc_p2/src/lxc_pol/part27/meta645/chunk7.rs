//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2214/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2214<F: Float>(t3082: F, t7586: F, t25641: F, t82892: F, t25638: F, t6735: F, t13532: F, t13537: F, t13797: F, t13941: F, t14122: F, t14126: F, t1920: F, t1941: F, t23548: F, t23564: F, t25679: F, t378: F, t4509: F, t7574: F, t7583: F, t82918: F, t82923: F, t83016: F, t83034: F, t83215: F) -> F {
    let t88479 = t7586 * t3082;
    let t88488 = F::cast_from(0.20186378047070195428e-3_f64) * t82892 * t25641;
    let t88503 = F::cast_from(0.20186378047070195428e-3_f64) * t25638 * t6735;
    let t88504 = t13941 * t1941 * t378 / F::cast_from(1536.0_f64) - t88479 / F::cast_from(6912.0_f64) + t1920 * t4509 * t13532 / F::cast_from(216.0_f64) + F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t1920 * t13797 * t13537 + t88488 - F::cast_from(0.10093189023535097714e-3_f64) * t83034 - F::cast_from(0.20186378047070195428e-3_f64) * t82918 * t7583 - F::cast_from(0.10093189023535097714e-3_f64) * t82923 * t7583 - F::cast_from(0.20186378047070195428e-3_f64) * t23564 * t25679 + t83016 * t14122 / F::cast_from(1152.0_f64) - t83215 * t14126 / F::cast_from(2304.0_f64) - F::cast_from(0.10093189023535097714e-3_f64) * t7574 * t23548 - t88503;
    t88504
}
