//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 614/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk614<F: Float>(t3507: F, t3544: F, t530: F, t186: F, t185: F, t1518: F, t495: F, t493: F, t543: F, t1279: F, t514: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3545 = t3507 + t3544;
    let t3546 = t530 * t3545;
    let t3547 = t186 * t3546;
    let t3549 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t185 * t3547;
    let t3550 = t1518 * t495;
    let t3551 = t493 * t3550;
    let t3552 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t3551;
    let t3553 = t1518 * t543;
    let t3554 = t185 * t3553;
    let t3555 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t3554;
    let t3556 = t514 * t1279;
    let t3557 = t185 * t3556;
    (t3545, t3546, t3547, t3549, t3550, t3551, t3552, t3553, t3554, t3555, t3556, t3557)
}
