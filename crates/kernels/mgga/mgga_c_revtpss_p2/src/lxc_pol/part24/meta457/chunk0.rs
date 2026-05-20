//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1426/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1426<F: Float>(t378: F, t53014: F, t11200: F, t1678: F, t11970: F, t1660: F, t127: F, t4823: F, t11239: F, t1647: F, t11245: F, t11255: F) -> (F, F, F, F, F, F, F) {
    let t53015 = t53014 * t378;
    let t53160 = t11200 * t1678;
    let t53326 = t1660 * t11970;
    let t53391 = t127 * t4823;
    let t53703 = t1647 * t11239;
    let t53704 = t53703 * t11245;
    let t53707 = t53703 * t11255;
    (t53015, t53160, t53326, t53391, t53703, t53704, t53707)
}
