//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 997/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk997<F: Float>(t13344: F, t4868: F, t571: F, t2018: F, t3742: F, t5285: F, t9678: F, t3669: F, t816: F, t9286: F, t10654: F, t1318: F, t2034: F, t2011: F, t13325: F, t13327: F, t13329: F, t13334: F, t13338: F, t13340: F, t13342: F) -> (F, F, F, F, F, F, F) {
    let t13347 = 16.0 / 3.0 * t571 * t4868 * t13344;
    let t13349 = 8.0 / 9.0 * t3742 * t2018;
    let t13351 = t571 * t9678 * t5285;
    let t13352 = 16.0 / 45.0 * t13351;
    let t13356 = 8.0 / 15.0 * t571 * t9286 * t816 * t3669;
    let t13358 = t1318 * t10654 * t2034;
    let t13359 = 16.0 / 135.0 * t13358;
    let t13361 = 8.0 / 15.0 * t3742 * t2011;
    let t13362 = -t13325 + t13327 - t13329 + t13334 + t13338 - t13340 - t13342 + t13347 + t13349 + t13352 - t13356 - t13359 - t13361;
    (t13347, t13349, t13352, t13356, t13359, t13361, t13362)
}
