//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1183/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1183<F: Float>(t23148: F, t30: F, t1583: F, t5962: F, t25207: F, t1544: F, t6079: F, t27383: F, t23429: F, t1468: F, t5966: F, t5824: F) -> (F, F, F, F, F, F, F, F) {
    let t113428 = t30 * t23148;
    let t113432 = t5962 * t1583;
    let t113433 = t25207 * t113432;
    let t113440 = t1544 * t6079;
    let t113441 = t27383 * t113440;
    let t113444 = t30 * t23429;
    let t113454 = t1468 * t5966;
    let t113461 = t5824 * t1544;
    (t113428, t113432, t113433, t113440, t113441, t113444, t113454, t113461)
}
