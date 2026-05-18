//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1093/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1093<F: Float>(t3228: F, t5047: F, t26896: F, t1021: F, t3448: F, t1096: F, t3452: F, t1196: F, t2825: F, t1200: F, t1189: F, t3178: F) -> (F, F, F, F, F, F, F) {
    let t26897 = t5047 * t3228;
    let t26898 = t26896 * t26897;
    let t26900 = t1021 * t3448;
    let t26902 = t1096 * t3452;
    let t26904 = t2825 * t1196;
    let t26906 = t2825 * t1200;
    let t26908 = t3178 * t1189;
    (t26897, t26898, t26900, t26902, t26904, t26906, t26908)
}
