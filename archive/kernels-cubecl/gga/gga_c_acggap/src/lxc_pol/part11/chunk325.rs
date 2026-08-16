//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 325/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk325<F: Float>(t1210: F, t150: F, t187: F, t119: F, t441: F, t186: F, t447: F) -> (F, F, F) {
    let t1212 = t1210 * t150 * t187;
    let t1215 = t119 * t441;
    let t1219 = F::cast_from(1.0_f64) / t447 / t186;
    (t1212, t1215, t1219)
}
