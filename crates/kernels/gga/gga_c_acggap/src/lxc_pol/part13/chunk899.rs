//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 899/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk899<F: Float>(t30714: F, t577: F, t7851: F, t339: F, t1181: F, t16507: F, t7351: F, t7426: F, t1165: F, t30327: F, t3355: F, t604: F) -> (F, F, F, F, F) {
    let t30715 = F::cast_from(0.12734375e-1_f64) * t30714;
    let t30716 = t7851 * t577;
    let t30717 = t30716 * t339;
    let t30721 = t7426 * t1181 * t7351 * t16507;
    let t30725 = t30327 * t1165 * t604 * t3355;
    (t30715, t30716, t30717, t30721, t30725)
}
