//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 109/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk109<F: Float>(t265: F, t75: F, t205: F, t207: F, t211: F, t216: F) -> (F, F) {
    let t266 = t75 * t265;
    let t271 = -0.86308333333333333334e0 * t205 - 0.301925e0 * t207 - 0.5501625e-1 * t211 - 0.82785e-1 * t216;
    (t266, t271)
}
