//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 365/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk365<F: Float>(t227: F, t2063: F, t229: F, t2062: F, t44: F, t650: F, t565: F, t695: F, sigma2: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t228 = t227 <= zeta_threshold;
    let t2066 = piecewise3(t228, 0.0, 4.0 / 3.0 * t229 * t2063);
    let t2068 = (t2062 + t2066) * t44;
    let t2355 = 1.0 / t650;
    let t2356 = sigma2 * t2355;
    let t2359 = piecewise3(t228, 0.0, t2063);
    let t2360 = t565 * t2359;
    let t2364 = t695 * t2063;
    (t2068, t2355, t2356, t2359, t2360, t2364)
}
