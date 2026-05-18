//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1016/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1016<F: Float>(t1082: F, t11173: F, t3298: F, t989: F, t3059: F, t3291: F, t4980: F, t994: F, t3151: F, t999: F, t3304: F, t4995: F) -> (F, F, F, F, F, F, F) {
    let t12111 = t1082 * t11173;
    let t12116 = t989 * t3298;
    let t12119 = t3291 * t3059;
    let t12122 = t994 * t4980;
    let t12123 = t999 * t3151;
    let t12124 = t12123 * t3304;
    let t12127 = t994 * t4995;
    (t12111, t12116, t12119, t12122, t12123, t12124, t12127)
}
