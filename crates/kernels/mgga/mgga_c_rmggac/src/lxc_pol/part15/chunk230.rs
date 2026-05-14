//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 230/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk230<F: Float>(t147: F, t362: F, t135: F, t376: F, t377: F) -> (F, F, F, F, F) {
    let t1088 = t362 * t147;
    let t1089 = 1.0 / t1088;
    let t1090 = t135 * t1089;
    let t1091 = t376 * t376;
    let t1092 = t1091 * t377;
    let t1094 = 2.0 * t1090 * t1092;
    (t1089, t1090, t1091, t1092, t1094)
}
