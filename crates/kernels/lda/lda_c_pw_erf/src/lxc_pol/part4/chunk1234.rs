//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1234/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1234<F: Float>(t1319: F, t16334: F, t571: F, t16300: F, t2017: F, t1472: F, t6367: F, t18295: F, t18296: F, t18300: F, t18302: F, t18306: F, t18309: F, t18312: F, t18315: F, t18318: F, t18320: F, t18324: F, t18328: F, t18330: F, t18333: F) -> (F, F, F, F) {
    let t18336 = 8.0 / 45.0 * t571 * t1319 * t16334;
    let t18339 = 8.0 / 9.0 * t571 * t2017 * t16300;
    let t18341 = 8.0 / 27.0 * t1472 * t6367;
    let t18342 = -t18295 + t18296 + t18300 - t18302 - t18306 - t18309 - t18312 - t18315 + t18318 - t18320 - t18324 - t18328 - t18330 - t18333 - t18336 - t18339 + t18341;
    (t18336, t18339, t18341, t18342)
}
