//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1045/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1045<F: Float>(t36985: F, t97: F, t1299: F, t3370: F, t1074: F, t6692: F, t1275: F, t502: F, t263: F, t6660: F, t321: F, t6100: F) -> (F, F, F, F, F, F) {
    let t36986 = t97 * t36985;
    let t37020 = t3370 * t1299;
    let t37023 = t1074 * t6692;
    let t37028 = t502 * t1275;
    let t37031 = t263 * t6660;
    let t37038 = t6100 * t321;
    (t36986, t37020, t37023, t37028, t37031, t37038)
}
