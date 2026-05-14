//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1019/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1019<F: Float>(t3863: F, t4769: F, t571: F, t4872: F, t4819: F, t9278: F, t3416: F, t5282: F, t1318: F, t3854: F, t4780: F, t3794: F, t5310: F, t1325: F, t3859: F, t4825: F) -> (F, F, F, F, F, F, F) {
    let t13255 = t571 * t3863 * t4769;
    let t13258 = t571 * t3863 * t4872;
    let t13261 = t571 * t9278 * t4819;
    let t13298 = t3416 * t5282;
    let t13301 = t1318 * t3854 * t4780;
    let t13303 = t3794 * t5310;
    let t13306 = t1325 * t3859 * t4825;
    (t13255, t13258, t13261, t13298, t13301, t13303, t13306)
}
