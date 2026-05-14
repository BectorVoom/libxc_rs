//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 702/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk702<F: Float>(t2593: F, t9165: F, t2585: F, t812: F, t2484: F, t2618: F, t2526: F, t808: F, t137: F, t8998: F, t161: F, t2491: F, t823: F, t2490: F, t2584: F, t754: F) -> (F, F, F, F, F, F, F, F) {
    let t9166 = t2593 * t9165;
    let t9168 = t2585 * t812;
    let t9170 = t2484 * t2618;
    let t9172 = t812 * t2526;
    let t9173 = t808 * t9172;
    let t9175 = t8998 * t137;
    let t9176 = t9175 * t161;
    let t9178 = t823 * t2491;
    let t9179 = t2490 * t9178;
    let t9181 = t2584 * t754;
    (t9166, t9168, t9170, t9173, t9175, t9176, t9179, t9181)
}
