//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1139/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1139<F: Float>(t14200: F, t15816: F, t519: F, t1446: F, t6423: F, t3863: F, t571: F, t6408: F, t3854: F, t6413: F, t4794: F, t6379: F, t13115: F, t4689: F, t6748: F, t4676: F, t6752: F) -> (F, F, F, F, F, F, F) {
    let t16758 = 352.0 / 243.0 * t519 * t14200 * t15816;
    let t16760 = 32.0 / 27.0 * t1446 * t6423;
    let t16762 = t571 * t3863 * t6408;
    let t16763 = 32.0 / 135.0 * t16762;
    let t16765 = t571 * t3854 * t6413;
    let t16766 = 32.0 / 45.0 * t16765;
    let t16768 = t571 * t4794 * t6379;
    let t16769 = 32.0 / 27.0 * t16768;
    let t16772 = 128.0 / 45.0 * t13115 * t6748 * t4689;
    let t16775 = 64.0 / 27.0 * t13115 * t6752 * t4676;
    (t16758, t16760, t16763, t16766, t16769, t16772, t16775)
}
