//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1121/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1121<F: Float>(t10613: F, t10592: F, t10596: F, t10604: F, t10611: F, t14442: F, t14443: F, t14444: F, t14615: F, t14618: F, t14620: F, t14621: F, t14624: F, t14626: F, t14628: F, t9542: F) -> (F, F) {
    let t14629 = 8.0 * t10613;
    let t14630 = t10592 + t14442 - t14443 - t10596 - t14444 - t10604 + t9542 + t14615 - t14618 + t14620 + t14621 + t14624 - t10611 + t14626 + t14628 + t14629;
    (t14629, t14630)
}
