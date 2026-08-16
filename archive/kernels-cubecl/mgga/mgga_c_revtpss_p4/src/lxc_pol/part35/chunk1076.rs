//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1076/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1076<F: Float>(t29547: F, t77: F, t5819: F, t603: F, t5826: F, t5816: F, t84: F, t30: F, t5966: F, t1544: F, t1583: F) -> (F, F, F, F, F, F) {
    let t29548 = t77 * t29547;
    let t29551 = t603 * t5819;
    let t29554 = t603 * t5826;
    let t29561 = t84 * t5816;
    let t29562 = t77 * t29561;
    let t29591 = t30 * t5966;
    let t29598 = t1544 * t1583;
    (t29548, t29551, t29554, t29562, t29591, t29598)
}
