//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 181/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk181<F: Float>(t5: F, t506: F, t129: F, t145: F, t369: F, t371: F, t504: F) -> (F, F, F) {
    let t507 = t5 * t506;
    let t509 = t129 * t507 * t145;
    let t513 = -t369 - F::cast_from(0.36675e0_f64) * t504 + t371;
    (t507, t509, t513)
}
