//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2040/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2040<F: Float>(t11752: F, t7111: F, t11755: F, t11937: F, t25500: F, t1024: F, t25553: F, t25495: F, t3215: F, t11817: F, t7117: F, t3223: F, t7125: F) -> (F, F, F, F, F, F, F) {
    let t93702 = t7111 * t11752;
    let t93704 = t7111 * t11755;
    let t93713 = t25500 * t11937;
    let t93715 = t1024 * t25553;
    let t93718 = t25495 * t3215;
    let t93720 = t7117 * t11817;
    let t93722 = t3223 * t7125;
    (t93702, t93704, t93713, t93715, t93718, t93720, t93722)
}
