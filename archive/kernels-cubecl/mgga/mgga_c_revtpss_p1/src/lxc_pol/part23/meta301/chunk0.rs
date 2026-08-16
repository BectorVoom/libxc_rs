//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1556/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1556<F: Float>(t12077: F, t342: F, t12051: F, t3154: F, t3298: F, t989: F, t4980: F, t994: F) -> (F, F, F, F) {
    let t12078 = t342 * t12077;
    let t12079 = t12051 * t3154;
    let t12116 = t989 * t3298;
    let t12122 = t994 * t4980;
    (t12078, t12079, t12116, t12122)
}
