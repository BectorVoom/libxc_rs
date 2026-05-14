//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1077/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1077<F: Float>(t1020: F, t3396: F, t568: F, t16193: F, t16230: F, t16273: F, t16275: F, t16280: F, t16283: F, t16287: F, t16290: F, t19624: F, t19688: F, t19690: F, t28914: F, t28916: F, t28917: F, t28918: F, t28919: F) -> (F, F, F) {
    let t29093 = t1020 * t3396;
    let t29094 = t29093 * t568;
    let t29111 = -t16193 + t28914 + t28916 - t16230 - t16273 + t16275 - t28917 + t19624 + t28918 + t28919 - t16280 + t16283 + t16287 - t16290 + t19688 + t19690;
    (t29093, t29094, t29111)
}
