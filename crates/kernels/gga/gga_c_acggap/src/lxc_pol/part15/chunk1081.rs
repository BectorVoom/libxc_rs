//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1081/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1081<F: Float>(t467: F, t9476: F, t1298: F, t560: F, t469: F, t5506: F, t157: F, t1914: F, t406: F, t1814: F, t33795: F, t615: F) -> (F, F, F, F, F, F) {
    let t38559 = t9476 * t467;
    let t38563 = t1298 * t560;
    let t38573 = t469 * t5506;
    let t38635 = t1914 * t406 * t157;
    let t38647 = t1814 * t406 * t157;
    let t38662 = t615 * t33795;
    (t38559, t38563, t38573, t38635, t38647, t38662)
}
