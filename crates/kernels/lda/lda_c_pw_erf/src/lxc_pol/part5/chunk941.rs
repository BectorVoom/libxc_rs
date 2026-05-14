//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 941/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk941<F: Float>(t20033: F, t40: F, t87: F, t390: F, t7376: F, t339: F, t7383: F, t344: F, t8195: F, t11335: F, t11325: F, t11328: F, t11333: F, t11338: F, t11340: F, t11341: F, t11342: F, t11343: F, t11344: F, t15413: F, t15421: F, t19987: F, t8202: F) -> (F, F, F, F, F, F, F) {
    let t20035 = t40 * t20033 * t87;
    let t20037 = t40 * t7376 * t390;
    let t20038 = t339 * t7383;
    let t20039 = 4.0 * t20038;
    let t20040 = t344 * t7383;
    let t20041 = 4.0 * t20040;
    let t20043 = 24.0 * t8195;
    let t20044 = 10.526802115419367 * t11335;
    let t20046 = -t19987 - 5.476843845342223 * t11325 + t11328 + t20035 + t20037 + t20039 - t20041 - 1.232289865202 * t15413 + t11333 - t20043 - t20044 + t11338 + t11340 + 2.0538164420033334 * t15421 - t11341 - t11342 - t8202 - t11343 - t11344;
    (t20035, t20037, t20039, t20041, t20043, t20044, t20046)
}
