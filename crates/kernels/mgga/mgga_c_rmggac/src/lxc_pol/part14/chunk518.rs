//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 518/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk518<F: Float>(t2021: F, t7335: F, t2020: F, t2165: F, t2019: F, t2169: F, t1311: F, t20: F, t2018: F) -> (F, F, F, F, F, F, F) {
    let t7336 = t7335 * t2021;
    let t7338 = t2020 * t2165;
    let t7339 = t2019 * t7338;
    let t7341 = t2020 * t2169;
    let t7342 = t2019 * t7341;
    let t7344 = t1311 * t20;
    let t7345 = t7344 * t2018;
    (t7336, t7338, t7339, t7341, t7342, t7344, t7345)
}
