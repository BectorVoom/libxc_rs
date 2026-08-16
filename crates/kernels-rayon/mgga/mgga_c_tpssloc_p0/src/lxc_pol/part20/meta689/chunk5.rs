//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2616/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2616(t3577: f64, t44951: f64, t4949: f64, t11692: f64, t1227: f64, t15615: f64, t15702: f64, t3578: f64, t45049: f64, t45114: f64, t4582: f64, t4728: f64, t484: f64, t48554: f64, t488: f64, t4978: f64, t52462: f64, t52897: f64, t53135: f64, t53142: f64, t53144: f64, t53149: f64, t53155: f64, t53158: f64, t68: f64) -> f64 {
    let t53161 = t3577 * t44951 * t4949;
    let t53162 = t53161 / 6912.0_f64;
    let t53167 = t52462 * t68 * t484 * t488 / 3072.0_f64 + t53135 / 1152.0_f64 - 5.0_f64 / 20736.0_f64 * t45049 - t1227 * t4582 * t15615 * t48554 / 256.0_f64 - t53142 / 288.0_f64 + t11692 * t3578 * t4728 * t53144 / 768.0_f64 + t11692 * t3578 * t53149 * t15702 / 1536.0_f64 - t53155 / 2304.0_f64 - t53158 / 1152.0_f64 + t53162 - 3.0_f64 / 512.0_f64 * t45114 * t52897 * t53149 * t4978;
    t53167
}
