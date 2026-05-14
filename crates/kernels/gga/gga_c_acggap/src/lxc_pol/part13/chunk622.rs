//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 622/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk622<F: Float>(t4128: F, t5336: F, t5357: F, t5390: F, t1662: F, t814: F, t467: F, t495: F, t406: F, t513: F, t157: F, t506: F, t407: F, t944: F, t1410: F, t1016: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5392 = t4128 + t5336 + t5357 + t5390;
    let t5399 = t1662 * t814;
    let t5439 = t495 * t467;
    let t5605 = t513 * t406;
    let t5606 = t5605 * t157;
    let t5615 = t506 * t406;
    let t5616 = t5615 * t157;
    let t5720 = t407 * t495;
    let t5746 = t944 * t495;
    let t5752 = t944 * t506;
    let t6263 = t944 * t1410;
    let t6337 = t1016 * t506;
    (t5392, t5399, t5439, t5606, t5616, t5720, t5746, t5752, t6263, t6337)
}
