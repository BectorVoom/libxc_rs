//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1452/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1452<F: Float>(t14524: F, t51297: F, t136: F, t2457: F, t39680: F, t6022: F, t10073: F, t18746: F, t18742: F, t10069: F, t2718: F, t6041: F) -> (F, F, F, F, F, F) {
    let t62874 = t51297 * t14524;
    let t62907 = t39680 * t6022 * t136 * t2457;
    let t62909 = t10073 * t18746;
    let t62920 = t10073 * t18742;
    let t62922 = t10069 * t18746;
    let t62929 = t2718 * t6041;
    (t62874, t62907, t62909, t62920, t62922, t62929)
}
