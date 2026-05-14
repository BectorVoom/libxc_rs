//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 818/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk818<F: Float>(t17859: F, t9218: F, t1907: F, t1971: F, t333: F, t511: F, t7230: F, t352: F, t515: F, t7717: F, t9783: F, t39277: F, t9123: F, t9206: F, t10014: F, t36662: F) -> (F, F, F, F, F, F, F) {
    let t45295 = t17859 * t9218;
    let t45300 = t7230 * t1971 * t511 * t1907 * t333;
    let t45305 = t7230 * t1971 * t515 * t1907 * t352;
    let t45307 = t7717 * t9783;
    let t45309 = t39277 * t9123;
    let t45316 = t39277 * t9206;
    let t45318 = t36662 * t10014;
    (t45295, t45300, t45305, t45307, t45309, t45316, t45318)
}
