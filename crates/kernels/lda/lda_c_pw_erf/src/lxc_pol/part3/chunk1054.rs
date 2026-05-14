//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1054/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1054<F: Float>(t4804: F, t4953: F, t1318: F, t1401: F, t1403: F, t34: F, t4892: F, t3794: F, t4946: F, t14307: F, t14311: F, t14314: F, t14317: F, t14319: F, t14321: F, t14323: F, t14327: F, t14329: F, t14331: F) -> (F, F, F, F) {
    let t14333 = 8.0 / 5.0 * t4804 * t4953;
    let t14338 = 8.0 / 5.0 * t1318 * t4892 * t1401 * t34 * t1403;
    let t14339 = t3794 * t4946;
    let t14340 = 16.0 / 15.0 * t14339;
    let t14341 = t14307 - t14311 + t14314 - t14317 + t14319 - t14321 - t14323 - t14327 - t14329 - t14331 + t14333 - t14338 - t14340;
    (t14333, t14338, t14340, t14341)
}
