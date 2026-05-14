//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 738/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk738<F: Float>(t360: F, t525: F, t1181: F, t604: F, t2068: F, t1165: F, t7351: F, t8906: F, t8402: F, t1967: F, t2310: F, t2290: F, t1089: F, t2090: F, t4643: F, t598: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8960 = t525 * t360;
    let t8962 = t1181 * t604 * t8960;
    let t8963 = t2068 * t8962;
    let t8966 = t1165 * t7351 * t8906;
    let t8967 = t2068 * t8966;
    let t8970 = t1165 * t604 * t8402;
    let t8971 = t2068 * t8970;
    let t8973 = t1967 * t2310;
    let t8975 = t1967 * t2290;
    let t8978 = t1089 * t4643 * t2090;
    let t8979 = t598 * t8978;
    (t8960, t8962, t8963, t8966, t8967, t8970, t8971, t8973, t8975, t8978, t8979)
}
