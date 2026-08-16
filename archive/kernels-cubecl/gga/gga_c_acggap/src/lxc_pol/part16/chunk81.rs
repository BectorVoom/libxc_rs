//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 81/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk81<F: Float>(t224: F, t88: F, t38: F, t7: F, t36: F) -> (F, F, F, F) {
    let t225 = t224 * t88;
    let t226 = F::cast_from(4.0_f64) * t225;
    let t227 = t38 * t7;
    let t228 = F::cast_from(1.0_f64) / t227;
    let t229 = t36 * t228;
    (t226, t227, t228, t229)
}
