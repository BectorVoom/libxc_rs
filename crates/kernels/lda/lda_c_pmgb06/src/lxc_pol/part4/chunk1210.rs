//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1210/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1210<F: Float>(t12304: F, t12307: F, t12308: F, t12310: F, t12312: F, t16297: F, t16299: F, t16300: F, t16301: F, t16302: F, t16303: F, t16306: F, t16308: F, t16310: F, t18225: F, t16313: F, t16315: F, t16321: F, t16336: F, t16338: F, t16339: F, t16340: F, t16342: F, t16345: F, t16347: F, t16349: F, t16351: F, t16353: F, t16357: F, t16362: F) -> (F, F) {
    let t18227 = -t16297 - t16299 + t16300 + t16301 + t16302 + t16303 - t16306 + 4.0 / 3.0 * t12304 + t12307 + 4.0 / 9.0 * t12308 + 16.0 / 9.0 * t12310 + 4e-21 * t12312 - t16308 + t16310 - 2.0 / 27.0 * t18225;
    let t18231 = -t16313 - t16315 - t16321 + t16336 + t16338 - t16339 - t16340 + t16342 + t16345 + t16347 + t16349 + t16351 + t16353 - t16357 - t16362;
    (t18227, t18231)
}
