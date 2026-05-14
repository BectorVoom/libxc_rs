//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 965/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk965<F: Float>(t11626: F, t3154: F, t357: F, t3129: F, t3172: F, t3127: F, t3135: F, t1041: F, t1024: F, t3105: F, t3151: F, t3153: F, t1052: F, t360: F, t3089: F, t1087: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11627 = 1.0 / t11626;
    let t11631 = t3154 * t357;
    let t11643 = t3172 * t3129;
    let t11644 = t3127 * t11643;
    let t11648 = t3172 * t3135;
    let t11649 = t1041 * t11648;
    let t11656 = t1024 * t3105;
    let t11659 = t3151 * t3153;
    let t11670 = t360 * t1052;
    let t11671 = t11670 * t3089;
    let t11672 = t1087 * t11671;
    (t11627, t11631, t11644, t11649, t11656, t11659, t11670, t11671, t11672)
}
