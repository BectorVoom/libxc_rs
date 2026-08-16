//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1307/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1307<F: Float>(t794: F, t852: F, t6562: F, t6572: F, t6552: F, t6555: F, t82124: F, t23035: F, t23237: F, t23241: F, t23219: F, t6547: F) -> (F, F, F, F, F) {
    let t82133 = t794 * t852;
    let t82135 = t6562 * t82133 * t6572;
    let t82138 = t6552 * t82124 * t6555;
    let t82141 = t23035 * t23237 * t23241;
    let t82143 = t6547 * t23219;
    (t82133, t82135, t82138, t82141, t82143)
}
