//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1176/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1176<F: Float>(t209: F, t7589: F, t7590: F, t9215: F, t26602: F, t26611: F, t92232: F, t2386: F, t7583: F, t92235: F, t26576: F, t26580: F) -> (F, F, F, F, F) {
    let t92325 = t7589 * t209 * t7590 * t9215;
    let t92327 = t26602 * t26611;
    let t92329 = t7589 * t92232;
    let t92332 = t2386 * t92235 * t7583;
    let t92334 = t26580 * t26576;
    (t92325, t92327, t92329, t92332, t92334)
}
