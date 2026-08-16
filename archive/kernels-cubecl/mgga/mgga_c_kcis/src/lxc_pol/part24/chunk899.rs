//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 899/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk899<F: Float>(t6436: F, t733: F, t6439: F, t1056: F, t18681: F, t1064: F, t1079: F, t6470: F, t743: F, t6461: F, t738: F, t6464: F) -> (F, F, F, F, F, F, F, F) {
    let t19423 = t733 * t6436;
    let t19425 = t733 * t6439;
    let t19427 = t1056 * t18681;
    let t19430 = t1064 * t18681;
    let t19433 = t1079 * t18681;
    let t19436 = t743 * t6470;
    let t19438 = t738 * t6461;
    let t19440 = t738 * t6464;
    (t19423, t19425, t19427, t19430, t19433, t19436, t19438, t19440)
}
