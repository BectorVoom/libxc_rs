//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 155/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk155<F: Float>(t407: F, t456: F, t182: F, t441: F, t119: F, t151: F, t451: F, t455: F) -> (F, F, F) {
    let t457 = t456 * t407;
    let t460 = t182 * t441;
    let t463 = t451 - t455 - F::cast_from(0.65854491829355115987e0_f64) * t151 * t457 + F::cast_from(0.65854491829355115987e0_f64) * t119 * t460;
    (t457, t460, t463)
}
