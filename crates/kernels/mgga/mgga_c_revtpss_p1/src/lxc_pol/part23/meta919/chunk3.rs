//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2967/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2967<F: Float>(t11256: F, t23642: F, t3172: F, t77492: F, t77494: F, t77496: F, t77498: F, t77600: F, t77604: F, t77612: F, t77622: F, t77624: F, t77628: F, t77634: F, t77636: F, t77639: F, t77641: F, t77643: F, t77645: F, t78402: F, t78405: F, t78411: F, t78413: F) -> (F, F) {
    let t78676 = t11256 * t3172 * t23642;
    let t78682 = -t77492 - t77494 - t77496 - t77498 + t77600 - t77604 + t78402 - t77612 + t77622 + t77624 + t77628 - t78405 - t78411 + t78413 + t77634 - t77636 + t77639 + t77641 + t77643 - t77645;
    (t78676, t78682)
}
