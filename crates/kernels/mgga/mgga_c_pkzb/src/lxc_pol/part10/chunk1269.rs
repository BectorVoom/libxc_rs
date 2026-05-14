//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1269/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1269<F: Float>(t16607: F, t16612: F, t19704: F, t24611: F, t24612: F, t24613: F, t24617: F, t24618: F, t24619: F, t24620: F, t24621: F, t24622: F, t8783: F, t192: F, t8817: F, t17245: F, t3501: F) -> (F, F, F) {
    let t24960 = 12.0 * t19704 * t8783 + t16607 - t16612 + t24611 - t24612 + t24613 - t24617 - t24618 + t24619 - t24620 + t24621 + t24622;
    let t24964 = t192 * t8817;
    let t24973 = t3501 * t17245;
    (t24960, t24964, t24973)
}
