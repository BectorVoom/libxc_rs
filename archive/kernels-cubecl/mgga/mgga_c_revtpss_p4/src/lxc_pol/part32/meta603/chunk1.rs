//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1940/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1940<F: Float>(t25266: F, t6019: F, t6024: F, t93054: F, t18495: F, t7045: F, t18500: F, t18618: F, t7038: F, t18466: F, t25270: F, t18622: F, t25245: F) -> (F, F, F, F, F, F, F) {
    let t106063 = t25266 * t6019;
    let t106065 = t93054 * t6024;
    let t106068 = t7045 * t18495;
    let t106070 = t7045 * t18500;
    let t106072 = t7038 * t18618;
    let t106074 = t25270 * t18466;
    let t106080 = t25245 * t18622;
    (t106063, t106065, t106068, t106070, t106072, t106074, t106080)
}
