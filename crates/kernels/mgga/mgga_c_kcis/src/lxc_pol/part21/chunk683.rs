//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 683/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk683<F: Float>(t157: F, t7627: F, t7615: F, t7618: F, t7620: F, t7622: F, t7625: F) -> (F, F) {
    let t7628 = t157 * t7627;
    let t7630 = t7615 / F::cast_from(8.0_f64) - t7618 / F::cast_from(8.0_f64) - t7620 / F::cast_from(4.0_f64) - t7622 / F::cast_from(32.0_f64) + t7625 / F::cast_from(32.0_f64) + t7628 / F::cast_from(8.0_f64);
    (t7628, t7630)
}
