//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 687/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk687<F: Float>(t32325: F, t469: F, t1317: F, t28: F, t375: F, t7256: F, t89: F, t358: F, t7165: F) -> (F, F, F, F, F) {
    let t32326 = t469 * t32325;
    let t32328 = t1317 * t28 * t32326;
    let t32331 = t89 * t375 * t7256;
    let t32332 = 2.0 / 9.0 * t32331;
    let t32333 = t7165 * t358;
    (t32326, t32328, t32331, t32332, t32333)
}
