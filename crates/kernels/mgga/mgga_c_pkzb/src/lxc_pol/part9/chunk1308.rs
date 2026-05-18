//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1308/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1308<F: Float>(t394: F, t6506: F, t178: F, t19080: F, t19079: F, t1227: F, t17938: F, t2370: F, t6460: F, t19091: F, t2401: F, t3185: F, t3188: F) -> (F, F, F, F, F, F, F, F) {
    let t22966 = t6506 * t394;
    let t22971 = t19080 * t178;
    let t22972 = t19079 * t22971;
    let t22973 = t1227 * t17938;
    let t22974 = t6460 * t2370;
    let t22979 = t19091 * t22971;
    let t22980 = t6460 * t394;
    let t22988 = t3185 * t2401 * t3188;
    (t22966, t22971, t22972, t22973, t22974, t22979, t22980, t22988)
}
