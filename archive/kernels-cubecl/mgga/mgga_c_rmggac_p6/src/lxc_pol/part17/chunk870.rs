//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 870/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk870<F: Float>(t1525: F, t1971: F, t209: F, t236: F, t605: F, t7453: F, t1970: F, t498: F, t6182: F, t7231: F, t321: F, t3352: F) -> (F, F, F) {
    let t44627 = t7453 * t1971 * t236 * t1525 * t605 * t209;
    let t44632 = t1970 * t7231 * t236 * t6182 * t498;
    let t44637 = t1970 * t3352 * t236 * t6182 * t321;
    (t44627, t44632, t44637)
}
