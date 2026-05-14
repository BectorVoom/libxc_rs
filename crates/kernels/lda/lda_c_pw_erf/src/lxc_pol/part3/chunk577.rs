//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 577/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk577<F: Float>(t186: F, t3546: F, t185: F, t1518: F, t495: F, t493: F, t543: F, t1279: F, t514: F, t1383: F, t565: F, t1284: F, t1289: F, t3464: F, t220: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3547 = t186 * t3546;
    let t3549 = 2.0 / 15.0 * t185 * t3547;
    let t3550 = t1518 * t495;
    let t3551 = t493 * t3550;
    let t3552 = 8.0 / 45.0 * t3551;
    let t3553 = t1518 * t543;
    let t3554 = t185 * t3553;
    let t3555 = 4.0 / 45.0 * t3554;
    let t3556 = t514 * t1279;
    let t3557 = t185 * t3556;
    let t3558 = 4.0 / 15.0 * t3557;
    let t3560 = 2.0 / 5.0 * t565 * t1383;
    let t3562 = 4.0 / 5.0 * t1284 * t1289;
    let t3563 = -t3464;
    let t3564 = t220 * t3563;
    let t3565 = t186 * t3564;
    (t3547, t3549, t3550, t3551, t3552, t3553, t3554, t3555, t3556, t3557, t3558, t3560, t3562, t3563, t3564, t3565)
}
