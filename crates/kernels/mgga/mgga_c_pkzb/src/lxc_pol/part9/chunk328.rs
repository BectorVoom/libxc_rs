//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 328/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk328<F: Float>(t1100: F, t261: F, t1068: F, t1074: F, t1077: F, t1081: F, t716: F, t719: F) -> (F, F) {
    let t1101 = t1100 * t261;
    let t1107 = F::new(0.258925e1) * t1074 - t716 + F::new(0.905775e0) * t1068 + F::new(0.16504875e0) * t1077 - t719 + F::new(0.248355e0) * t1081;
    (t1101, t1107)
}
