//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 615/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk615<F: Float>(t3557: F, t1383: F, t565: F, t1284: F, t1289: F, t3464: F, t220: F, t186: F, t548: F, t3442: F, t3444: F, t3447: F, t3449: F, t3451: F, t3453: F, t3457: F, t3459: F, t3461: F, t3463: F, t3468: F, t3549: F, t3552: F, t3555: F) -> (F, F, F, F, F, F, F, F) {
    let t3558 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t3557;
    let t3560 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t565 * t1383;
    let t3562 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t1284 * t1289;
    let t3563 = -t3464;
    let t3564 = t220 * t3563;
    let t3565 = t186 * t3564;
    let t3567 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t548 * t3565;
    let t3568 = t3442 + t3444 + t3447 - t3449 + t3451 - t3453 + t3457 - t3459 - t3461 + t3463 + t3468 - t3549 - t3552 + t3555 - t3558 - t3560 + t3562 + t3567;
    (t3558, t3560, t3562, t3563, t3564, t3565, t3567, t3568)
}
