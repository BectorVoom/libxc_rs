//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1434/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1434<F: Float>(t17633: F, t3629: F, t3626: F, t2258: F, t3628: F, t5351: F, t3367: F, t471: F, t2251: F, t372: F, t5296: F, t5297: F, t5405: F) -> (F, F, F, F, F) {
    let t17634 = t17633 * t3629;
    let t17635 = t3626 * t17634;
    let t17638 = t3628 * t2258;
    let t17639 = t5351 * t17638;
    let t17640 = t3626 * t17639;
    let t17643 = t471 * t3367;
    let t17644 = t17643 * t2251;
    let t17645 = t5351 * t17644;
    let t17646 = t3626 * t17645;
    let t17649 = t372 * t5296;
    let t17650 = t5297 * t5405;
    (t17635, t17640, t17646, t17649, t17650)
}
