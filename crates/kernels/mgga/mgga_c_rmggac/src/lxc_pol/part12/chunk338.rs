//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 338/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk338<F: Float>(t2024: F, t352: F, t321: F, t665: F, t333: F, t645: F, t1343: F, t36: F, t71: F) -> (F, F, F, F, F, F) {
    let t2025 = t2024 * t352;
    let t2028 = t665 * t321;
    let t2031 = t665 * t333;
    let t2034 = t645 * t321;
    let t2038 = t36 * t1343;
    let t2039 = t2038 * t71;
    (t2025, t2028, t2031, t2034, t2038, t2039)
}
