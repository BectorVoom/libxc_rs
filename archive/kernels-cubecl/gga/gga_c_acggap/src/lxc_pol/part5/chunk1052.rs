//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1052/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1052<F: Float>(t4389: F, t5143: F, t1165: F, t1532: F, t301: F, t3194: F, t4162: F, t4542: F, t997: F, t12719: F, t527: F, t3361: F, t3809: F, t540: F) -> (F, F, F, F, F) {
    let t18426 = t4389 * t5143;
    let t18436 = t3194 * t1165 * t1532 * t4162 * t301;
    let t18458 = t997 * t4542;
    let t18460 = t12719 * t527;
    let t18475 = t3361 * t1165 * t540 * t3809;
    (t18426, t18436, t18458, t18460, t18475)
}
