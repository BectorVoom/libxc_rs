//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 846/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk846<F: Float>(t12261: F, t2024: F, t782: F, t2020: F, t4597: F, t1990: F, t5444: F, t2038: F, t5531: F, t2040: F, t798: F) -> (F, F, F, F, F, F, F) {
    let t12262 = t12261 * t2024;
    let t12263 = t782 * t12262;
    let t12271 = t2020 * t4597;
    let t12325 = t1990 * t5444;
    let t12345 = t2038 * t5531;
    let t12350 = t2040 * t2040;
    let t12351 = 1.0 / t12350;
    let t12352 = t798 * t12351;
    (t12263, t12271, t12325, t12345, t12350, t12351, t12352)
}
