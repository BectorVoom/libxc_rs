//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1040/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1040<F: Float>(t2533: F, t26651: F, t2153: F, t2538: F, t9312: F, t31274: F, t7612: F, t26607: F, t26623: F, t26620: F, t700: F, t9236: F, t7580: F, t26597: F, t9251: F, t2387: F) -> (F, F, F, F, F, F, F, F) {
    let t92165 = 3.0 * t2533 * t26651;
    let t92168 = 2.0 * t2538 * t2153 * t9312;
    let t92170 = 6.0 * t31274 * t7612;
    let t92171 = t26607 * t26623;
    let t92174 = t26620 * t9236 * t700;
    let t92175 = t7580 * t92174;
    let t92177 = t26597 * t26623;
    let t92179 = t9251 * t700;
    let t92181 = t26620 * t92179 * t2387;
    (t92165, t92168, t92170, t92171, t92174, t92175, t92177, t92181)
}
