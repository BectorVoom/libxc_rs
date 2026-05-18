//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1179/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1179<F: Float>(t11888: F, t6654: F, t1276: F, t2391: F, t3366: F, t1070: F, t8395: F, t11047: F, t23498: F, t11050: F, t8358: F, t11885: F) -> (F, F, F, F, F, F) {
    let t40794 = t6654 * t11888;
    let t40797 = t1276 * t3366 * t2391;
    let t40800 = t1276 * t1070 * t8395;
    let t40802 = t23498 * t11047;
    let t40804 = t8358 * t11050;
    let t40806 = t6654 * t11885;
    (t40794, t40797, t40800, t40802, t40804, t40806)
}
