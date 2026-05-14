//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 955/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk955<F: Float>(t25920: F, t25965: F, t416: F, t467: F, t471: F, t415: F, t25413: F, t5625: F, t3484: F, t19740: F, t3973: F, t8044: F, t1309: F, t6157: F, t6171: F, t1313: F, t25312: F) -> (F, F, F, F, F, F, F, F) {
    let t25966 = t25920 + t25965;
    let t25967 = t416 * t25966;
    let t25968 = t25967 * t467;
    let t25969 = t25968 * t471;
    let t25970 = t415 * t25969;
    let t25972 = t5625 * t25413;
    let t25973 = t3484 * t25972;
    let t25974 = t19740 * t25973;
    let t25980 = t3973 * t8044;
    let t25981 = t1309 * t25980;
    let t25985 = t6157 * t6171;
    let t25993 = t1313 * t25312;
    (t25966, t25967, t25970, t25972, t25974, t25981, t25985, t25993)
}
