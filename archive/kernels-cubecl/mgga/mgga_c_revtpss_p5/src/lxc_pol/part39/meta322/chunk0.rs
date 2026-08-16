//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1097/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1097<F: Float>(t1043: F, t3153: F, t3133: F, t4982: F, t3046: F, t3286: F, t3057: F, t1071: F, t1086: F, t994: F, t3316: F, t989: F) -> (F, F, F, F, F, F) {
    let t12131 = t1043 * t3153;
    let t12132 = t4982 * t3133;
    let t12146 = t3046 * t3286;
    let t12149 = t3057 * t3286;
    let t12153 = t1086 * t1071;
    let t12154 = t994 * t12153;
    let t12160 = t989 * t3316;
    (t12131, t12132, t12146, t12149, t12154, t12160)
}
