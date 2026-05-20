//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1340/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1340<F: Float>(t11239: F, t3143: F, t342: F, t3298: F, t989: F, t4980: F, t994: F, t4995: F, t1043: F, t3153: F, t3046: F, t3286: F) -> (F, F, F, F, F, F, F) {
    let t12077 = t11239 * t3143;
    let t12078 = t342 * t12077;
    let t12116 = t989 * t3298;
    let t12122 = t994 * t4980;
    let t12127 = t994 * t4995;
    let t12131 = t1043 * t3153;
    let t12146 = t3046 * t3286;
    (t12077, t12078, t12116, t12122, t12127, t12131, t12146)
}
