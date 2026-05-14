//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 680/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk680<F: Float>(t2185: F, t7690: F, t271: F, t4765: F, t4768: F, t7325: F, t2164: F, t7323: F, t7324: F, t1327: F, t356: F, t640: F, t507: F, t8619: F, t22: F, t235: F, t29837: F) -> (F, F, F, F, F, F) {
    let t34902 = t7690 * t2185;
    let t34921 = t4765 * t4768 * t271 * t7325;
    let t34922 = 0.64980365807044550255e-5 * t34921;
    let t34927 = t7323 * t2164 * t7324;
    let t34931 = t7323 * t640 * t356 * t1327;
    let t34938 = t507 * t8619;
    let t34944 = t235 * t29837 * t22;
    (t34902, t34922, t34927, t34931, t34938, t34944)
}
