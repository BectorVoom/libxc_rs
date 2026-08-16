//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1361/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1361<F: Float>(t11643: F, t3127: F, t3135: F, t3172: F, t1041: F, t1024: F, t3105: F, t3151: F, t3153: F, t1052: F, t360: F, t3089: F) -> (F, F, F, F, F, F, F) {
    let t11644 = t3127 * t11643;
    let t11648 = t3172 * t3135;
    let t11649 = t1041 * t11648;
    let t11656 = t1024 * t3105;
    let t11659 = t3151 * t3153;
    let t11670 = t360 * t1052;
    let t11671 = t11670 * t3089;
    (t11644, t11648, t11649, t11656, t11659, t11670, t11671)
}
