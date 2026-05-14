//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 580/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk580<F: Float>(t2453: F, t942: F, t2422: F, t2430: F, t411: F, t415: F, t938: F, t952: F) -> (F, F) {
    let t2454 = t942 * t2453;
    let t2457 = 0.65854491829355115987e0 * t2422 * t415 - 0.13170898365871023197e1 * t938 * t952 + 0.13170898365871023197e1 * t411 * t2430 - 0.65854491829355115987e0 * t411 * t2454;
    (t2454, t2457)
}
