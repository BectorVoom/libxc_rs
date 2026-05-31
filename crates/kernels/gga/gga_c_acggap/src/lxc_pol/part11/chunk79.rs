//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 79/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk79<F: Float>(t205: F, t207: F, t211: F, t216: F, t30: F) -> (F, F) {
    let t218 = -F::cast_from(0.632975e0_f64) * t205 - F::cast_from(0.29896666666666666667e0_f64) * t207 - F::cast_from(0.1023875e0_f64) * t211 - F::cast_from(0.82156666666666666667e-1_f64) * t216;
    let t219 = F::cast_from(1.0_f64) / t30;
    (t218, t219)
}
