//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 625/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk625<F: Float>(t211: F, t3437: F, t1284: F, t1397: F, t1404: F, t514: F, t1506: F, t172: F, t184: F, t1234: F, t511: F, t191: F, t717: F, t187: F, t190: F, t1272: F, t331: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3439 = 16.0 / 405.0 * t211 * t3437;
    let t3443 = t1284 * t1397;
    let t3445 = t514 * t1404;
    let t3446 = t211 * t3445;
    let t3454 = t172 * t1506;
    let t3455 = t3454 * t184;
    let t3458 = t511 * t1234;
    let t3469 = t717 * t191;
    let t3472 = 0.02962962962962963 * t190 * t3469 * t187;
    let t3473 = t331 * t1272;
    (t3439, t3443, t3445, t3446, t3454, t3455, t3458, t3469, t3472, t3473)
}
