//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 507/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk507<F: Float>(t130: F, t20: F, t21: F, t736: F, t15: F, t97: F, t787: F, t5: F, t728: F, t88: F, t4: F, t66: F, t789: F, t128: F, t717: F, t2440: F) -> (F, F, F, F, F, F, F, F) {
    let t2553 = t130 * t20;
    let t2555 = t2553 * t21 * t736;
    let t2558 = t15 * t97;
    let t2559 = t787 * t2558;
    let t2561 = t5 * t88 * t728;
    let t2565 = t789 * t4 * t66;
    let t2568 = t128 * t717;
    let t2569 = t2568 * t2440;
    (t2553, t2555, t2558, t2559, t2561, t2565, t2568, t2569)
}
