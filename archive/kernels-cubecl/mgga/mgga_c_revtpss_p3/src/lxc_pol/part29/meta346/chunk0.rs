//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1270/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1270<F: Float>(t3259: F, t359: F, t11239: F, t3143: F, t342: F, t1086: F, t3043: F, t3298: F, t989: F, t4980: F, t994: F, t4995: F) -> (F, F, F, F, F, F, F) {
    let t12073 = t359 * t3259;
    let t12077 = t11239 * t3143;
    let t12078 = t342 * t12077;
    let t12097 = t3043 * t1086;
    let t12116 = t989 * t3298;
    let t12122 = t994 * t4980;
    let t12127 = t994 * t4995;
    (t12073, t12077, t12078, t12097, t12116, t12122, t12127)
}
