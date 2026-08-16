//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2616/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2616<F: Float>(t3577: F, t44951: F, t4949: F, t11692: F, t1227: F, t15615: F, t15702: F, t3578: F, t45049: F, t45114: F, t4582: F, t4728: F, t484: F, t48554: F, t488: F, t4978: F, t52462: F, t52897: F, t53135: F, t53142: F, t53144: F, t53149: F, t53155: F, t53158: F, t68: F) -> F {
    let t53161 = t3577 * t44951 * t4949;
    let t53162 = t53161 / F::cast_from(6912.0_f64);
    let t53167 = t52462 * t68 * t484 * t488 / F::cast_from(3072.0_f64) + t53135 / F::cast_from(1152.0_f64) - F::cast_from(5.0_f64) / F::cast_from(20736.0_f64) * t45049 - t1227 * t4582 * t15615 * t48554 / F::cast_from(256.0_f64) - t53142 / F::cast_from(288.0_f64) + t11692 * t3578 * t4728 * t53144 / F::cast_from(768.0_f64) + t11692 * t3578 * t53149 * t15702 / F::cast_from(1536.0_f64) - t53155 / F::cast_from(2304.0_f64) - t53158 / F::cast_from(1152.0_f64) + t53162 - F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t45114 * t52897 * t53149 * t4978;
    t53167
}
