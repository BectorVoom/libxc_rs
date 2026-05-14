//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 523/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk523<F: Float>(t236: F, t4545: F, t1971: F, t7365: F, t4510: F, t1970: F, t352: F, t498: F, t515: F, t7231: F, t3351: F, t4048: F, t3352: F, t2157: F, t892: F, t132: F, t1338: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7366 = t236 * t4545;
    let t7367 = t1971 * t7366;
    let t7368 = t7365 * t7367;
    let t7370 = t236 * t4510;
    let t7371 = t1971 * t7370;
    let t7372 = t1970 * t7371;
    let t7374 = t352 * t498;
    let t7375 = t515 * t7374;
    let t7376 = t7231 * t7375;
    let t7377 = t3351 * t7376;
    let t7379 = t515 * t4048;
    let t7380 = t3352 * t7379;
    let t7381 = t3351 * t7380;
    let t7383 = t892 * t2157;
    let t7385 = t132 * t1338;
    (t7367, t7368, t7371, t7372, t7376, t7377, t7380, t7381, t7383, t7385)
}
