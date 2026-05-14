//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 427/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk427<F: Float>(t545: F, t555: F, t869: F, t689: F, t546: F, t786: F) -> (F, F, F, F) {
    let t1428 = t545 * t555;
    let t1429 = t869 * t1428;
    let t1431 = 0.54878743191129263322e-2 * t689 * t1429;
    let t1432 = t786 * t546;
    (t1428, t1429, t1431, t1432)
}
