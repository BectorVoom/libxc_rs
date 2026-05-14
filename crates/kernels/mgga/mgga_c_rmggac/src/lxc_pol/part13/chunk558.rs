//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 558/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk558<F: Float>(t1971: F, t7745: F, t3351: F, t352: F, t495: F, t515: F, t7230: F, t1343: F, t49: F, t288: F) -> (F, F, F, F, F, F) {
    let t7746 = t1971 * t7745;
    let t7747 = t3351 * t7746;
    let t7750 = t515 * t352 * t495;
    let t7751 = t1971 * t7750;
    let t7752 = t7230 * t7751;
    let t7754 = t1343 * t49;
    let t7755 = t7754 * t288;
    (t7746, t7747, t7751, t7752, t7754, t7755)
}
