//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 667/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk667<F: Float>(t157: F, t5615: F, t407: F, t495: F, t944: F, t506: F, t1410: F, t1016: F, t469: F, t624: F) -> (F, F, F, F, F, F, F) {
    let t5616 = t5615 * t157;
    let t5720 = t407 * t495;
    let t5746 = t944 * t495;
    let t5752 = t944 * t506;
    let t6263 = t944 * t1410;
    let t6337 = t1016 * t506;
    let t7278 = t624 * t469;
    (t5616, t5720, t5746, t5752, t6263, t6337, t7278)
}
