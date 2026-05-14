//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 139/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk139<F: Float>(t115: F, t118: F, t1: F, t463: F, t125: F, t3: F, t128: F) -> (F, F, F, F, F) {
    let t464 = t115 * t118;
    let t465 = t464 * t1;
    let t466 = t463 * t465;
    let t467 = t3 * t125;
    let t468 = t467 * t128;
    (t464, t465, t466, t467, t468)
}
