//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 962/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk962<F: Float>(t39490: F, t8571: F, t2144: F, t3351: F, t498: F, t6557: F, t7231: F, t3352: F, t6583: F, t1971: F, t6586: F, t7190: F) -> (F, F, F, F) {
    let t45951 = t8571 * t39490;
    let t45956 = t3351 * t7231 * t2144 * t6557 * t498;
    let t45960 = t3351 * t3352 * t2144 * t6583;
    let t45964 = t3351 * t1971 * t7190 * t6586;
    (t45951, t45956, t45960, t45964)
}
