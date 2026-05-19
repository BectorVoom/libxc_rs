//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 31/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk31<F: Float>(t40: F, t88: F, t60: F, t85: F) -> (F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t89 = t40 * t88;
    let t91 = F::cast_from(0.19751673498613801407e-1_f64) * t60 * t85;
    let t92 = F::ln(F::new(2.0));
    let t93 = F::new(1.0) - t92;
    let t94 = pi * pi;
    (t89, t91, t93, t94)
}
