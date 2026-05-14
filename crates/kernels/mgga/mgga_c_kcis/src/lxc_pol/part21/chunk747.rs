//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 747/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk747<F: Float>(t1071: F, t253: F, t1017: F, t86: F, t2843: F, t329: F, t2822: F, t2826: F, t2831: F, t2820: F, t2840: F, t2847: F, t3225: F, t283: F, t3201: F, t982: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9368 = 1.0 / t253 / t1071;
    let t9370 = t86 * t1017 * t9368;
    let t9372 = 1.0 / t2843 / t329;
    let t9379 = t2822 * t2826;
    let t9383 = t2822 * t2831;
    let t9386 = t86 * t2820 * t2840;
    let t9387 = t9386 * t2847;
    let t9409 = t3225 * sigma0;
    let t9410 = t9409 * t283;
    let t9415 = t3201 * t982;
    (t9368, t9370, t9372, t9379, t9383, t9386, t9387, t9409, t9410, t9415)
}
