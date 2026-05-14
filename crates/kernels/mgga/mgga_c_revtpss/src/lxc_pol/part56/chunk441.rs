//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 441/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk441<F: Float>(t357: F, t1038: F, t1052: F, t1036: F, t1033: F, t127: F, t246: F, t1046: F, t1041: F, t283: F, t905: F, t1020: F, t1062: F, t1021: F, t1058: F, t371: F, t373: F, t676: F) -> (F, F, F, F, F, F, F, F) {
    let t3154 = t357 * t357;
    let t3167 = t1052 * t1038;
    let t3168 = t1036 * t3167;
    let t3169 = t1033 * t3168;
    let t3172 = t246 * t127;
    let t3173 = t3172 * t1046;
    let t3174 = t1041 * t3173;
    let t3181 = 1.0 / t283 / t905;
    let t3188 = t1020 * t1062;
    let t3194 = t1021 * t1058;
    let t3201 = t371 * t676 * t373;
    (t3154, t3169, t3172, t3174, t3181, t3188, t3194, t3201)
}
