//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 994/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk994<F: Float>(t13298: F, t1318: F, t3854: F, t4780: F, t3794: F, t5310: F, t1325: F, t3859: F, t4825: F, t12695: F, t4830: F, t1278: F, t5289: F, t542: F, t784: F, t1997: F, t3709: F) -> (F, F, F, F, F, F, F) {
    let t13299 = 32.0 / 45.0 * t13298;
    let t13301 = t1318 * t3854 * t4780;
    let t13302 = 16.0 / 45.0 * t13301;
    let t13303 = t3794 * t5310;
    let t13304 = 32.0 / 45.0 * t13303;
    let t13306 = t1325 * t3859 * t4825;
    let t13307 = 16.0 / 45.0 * t13306;
    let t13309 = t1325 * t12695 * t4830;
    let t13310 = 16.0 / 9.0 * t13309;
    let t13315 = 8.0 / 5.0 * t1325 * t5289 * t784 * t542 * t1278;
    let t13317 = 4.0 / 15.0 * t3709 * t1997;
    (t13299, t13302, t13304, t13307, t13310, t13315, t13317)
}
