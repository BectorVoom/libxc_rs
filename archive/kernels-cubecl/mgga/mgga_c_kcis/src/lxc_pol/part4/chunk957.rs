//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 957/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk957<F: Float>(t2847: F, t9386: F, t3225: F, t283: F, t3201: F, t982: F, t1018: F, t1085: F, t1017: F, t86: F, sigma0: F) -> (F, F, F, F, F) {
    let t9387 = t9386 * t2847;
    let t9409 = t3225 * sigma0;
    let t9410 = t9409 * t283;
    let t9415 = t3201 * t982;
    let t9423 = t1018 * t1085;
    let t9425 = t86 * t1017 * t9423;
    (t9387, t9409, t9410, t9415, t9425)
}
