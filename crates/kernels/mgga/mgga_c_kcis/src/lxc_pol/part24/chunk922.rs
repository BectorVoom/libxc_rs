//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 922/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk922<F: Float>(t19679: F, t4580: F, t14381: F, t3200: F, t1014: F, t6483: F, t1133: F, t6555: F, t3218: F, t1021: F, t1092: F, t1121: F) -> (F, F, F, F, F, F) {
    let t19750 = t4580 * t19679;
    let t19751 = t14381 * t19750;
    let t19752 = t3200 * t19751;
    let t19754 = t1014 * t6483;
    let t19756 = t6555 * t1133;
    let t19757 = t3218 * t19756;
    let t19758 = t1021 * t19757;
    let t19759 = t1092 * t19758;
    let t19763 = t6555 * t1121;
    (t19750, t19752, t19754, t19756, t19759, t19763)
}
