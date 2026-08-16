//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3719/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3719<F: Float>(t3671: F, t371: F, t6609: F, t676: F, t5245: F, t1235: F, t127: F, t21083: F, t12967: F, t20846: F, t17708: F, t59550: F) -> (F, F, F, F, F) {
    let t70511 = t3671 * t371 * t676 * t6609;
    let t70513 = t5245 * t5245;
    let t70521 = t1235 * t371 * t127 * t21083;
    let t70523 = t12967 * t20846;
    let t70530 = t59550 * t17708;
    (t70511, t70513, t70521, t70523, t70530)
}
