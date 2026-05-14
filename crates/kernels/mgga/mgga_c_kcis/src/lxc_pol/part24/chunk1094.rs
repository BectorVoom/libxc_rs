//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1094/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1094<F: Float>(t26938: F, t29062: F, t26924: F, t99945: F, t99948: F, t99950: F, t99952: F, t99954: F, t99956: F, t99958: F, t99960: F, t99962: F, t99964: F, t99966: F, t99968: F, t99970: F, t99972: F, t99974: F, t99977: F) -> (F, F, F) {
    let t99979 = t26938 * t29062;
    let t99981 = t26924 * t29062;
    let t99983 = -t99945 / 48.0 - t99948 / 144.0 - t99950 / 9.0 + t99952 / 128.0 - t99954 / 12.0 - t99956 / 12.0 - t99958 / 12.0 - t99960 / 24.0 + t99962 / 54.0 - t99964 / 24.0 - t99966 / 16.0 - t99968 / 3.0 - t99970 / 12.0 + t99972 / 54.0 + t99974 / 48.0 + t99977 / 24.0 + t99979 / 24.0 - t99981 / 9.0;
    (t99979, t99981, t99983)
}
