//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1243/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1243<F: Float>(t17638: F, t5351: F, t3626: F, t3367: F, t471: F, t2251: F, t372: F, t5296: F, t5297: F, t5405: F, t17350: F, t3767: F, t1121: F, t1248: F, t606: F, t3604: F) -> (F, F, F, F, F) {
    let t17639 = t5351 * t17638;
    let t17640 = t3626 * t17639;
    let t17643 = t471 * t3367;
    let t17644 = t17643 * t2251;
    let t17645 = t5351 * t17644;
    let t17646 = t3626 * t17645;
    let t17649 = t372 * t5296;
    let t17650 = t5297 * t5405;
    let t17651 = t17649 * t17650;
    let t17654 = t3767 * t17350;
    let t17655 = t1248 * t1121;
    let t17656 = t17655 * t606;
    let t17657 = t3604 * t17656;
    (t17640, t17646, t17651, t17654, t17657)
}
