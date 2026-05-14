//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 922/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk922<F: Float>(t10065: F, t2782: F, t123: F, t212: F, t2434: F, t4089: F, t138: F, t2438: F, t785: F, t1398: F, t1419: F, t4086: F, t543: F, t4056: F, t555: F, t1432: F, t2470: F, t4107: F) -> (F, F, F, F, F, F, F, F) {
    let t10066 = t2782 * t10065;
    let t10069 = t123 * t2434 * t212;
    let t10070 = t10069 * t4089;
    let t10073 = t138 * t2438 * t785;
    let t10074 = t10073 * t4089;
    let t10079 = t4086 * t1419 * t1398 * t543;
    let t10080 = t2782 * t10079;
    let t10082 = t555 * t4056;
    let t10084 = t4086 * t10082 * t543;
    let t10085 = t2782 * t10084;
    let t10098 = t1432 * t4107 * t2470;
    (t10066, t10069, t10070, t10073, t10074, t10080, t10085, t10098)
}
