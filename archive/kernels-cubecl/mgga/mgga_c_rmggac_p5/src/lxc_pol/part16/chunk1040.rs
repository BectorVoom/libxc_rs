//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1040/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1040<F: Float>(t1950: F, t2164: F, t638: F, t639: F, t640: F, t6617: F, t1971: F, t236: F, t495: F, t6172: F, t7453: F, t1951: F, t2046: F, t2050: F, t31: F) -> (F, F, F, F) {
    let t47676 = t638 * t639 * t2164 * t1950;
    let t47680 = t638 * t639 * t640 * t6617;
    let t47690 = t7453 * t1971 * t236 * t6172 * t495;
    let t47694 = t2046 * t2050 * t1951 * t31;
    (t47676, t47680, t47690, t47694)
}
