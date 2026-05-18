//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1063/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1063<F: Float>(t1907: F, t236: F, t498: F, t7230: F, t7231: F, t1652: F, t1971: F, t515: F, t615: F, t2144: F, t495: F, t6557: F) -> (F, F, F) {
    let t47455 = t7230 * t7231 * t236 * t1907 * t498;
    let t47460 = t7230 * t1971 * t515 * t1652 * t615;
    let t47465 = t7230 * t1971 * t2144 * t6557 * t495;
    (t47455, t47460, t47465)
}
