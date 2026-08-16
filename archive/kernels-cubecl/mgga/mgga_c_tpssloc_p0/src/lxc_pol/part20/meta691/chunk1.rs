//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2624/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2624<F: Float>(t1213: F, t15525: F, t248: F, t3570: F, t11813: F, t5018: F, t15749: F, t3577: F, t45124: F, t11734: F, t1214: F, t1218: F, t15531: F, t15553: F, t3494: F, t3515: F, t3518: F, t4582: F, t475: F, t52458: F, t53378: F, t53387: F, t53389: F, t53397: F, t53399: F) -> F {
    let t53404 = t1213 * t248 * t3570 * t15525;
    let t53406 = t11813 * t5018;
    let t53410 = t3577 * t45124 * t15749;
    let t53412 = -t53378 / F::cast_from(768.0_f64) - t11734 * t15531 / F::cast_from(1024.0_f64) - t3515 * t4582 * t15553 * t3494 / F::cast_from(1024.0_f64) - t53387 / F::cast_from(72.0_f64) - t53389 / F::cast_from(288.0_f64) + t1213 * t248 * t1214 * t52458 * t475 / F::cast_from(3072.0_f64) + t53397 / F::cast_from(1536.0_f64) - t53399 * t3518 / F::cast_from(1024.0_f64) + t53404 / F::cast_from(1536.0_f64) - t53406 * t1218 / F::cast_from(192.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t53410;
    t53412
}
