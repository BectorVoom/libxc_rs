//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 627/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk627<F: Float>(t157: F, t7627: F, t7615: F, t7618: F, t7620: F, t7622: F, t7625: F) -> (F, F) {
    let t7628 = t157 * t7627;
    let t7630 = t7615 / 8.0 - t7618 / 8.0 - t7620 / 4.0 - t7622 / 32.0 + t7625 / 32.0 + t7628 / 8.0;
    (t7628, t7630)
}
