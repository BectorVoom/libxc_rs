//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1096/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1096<F: Float>(t1176: F, t6681: F, t1021: F, t20191: F, t19576: F, t95474: F, t19885: F, t3227: F, t5099: F, t95381: F, t19926: F, t7748: F, t19934: F, t19931: F, t92447: F, t5048: F, t95351: F) -> (F, F, F, F, F, F, F, F, F) {
    let t100001 = t6681 * t1176;
    let t100003 = t1021 * t20191;
    let t100005 = t95474 * t19576;
    let t100007 = t3227 * t19885;
    let t100009 = t95381 * t5099;
    let t100011 = t7748 * t19926;
    let t100013 = t7748 * t19934;
    let t100015 = t92447 * t19931;
    let t100017 = t95351 * t5048;
    (t100001, t100003, t100005, t100007, t100009, t100011, t100013, t100015, t100017)
}
