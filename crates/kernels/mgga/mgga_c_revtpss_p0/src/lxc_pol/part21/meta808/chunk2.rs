//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2949/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2949<F: Float>(t11620: F, t1651: F, t11710: F, t15969: F, t4892: F, t1062: F, t15655: F, t11239: F, t1647: F, t11245: F, t11255: F, t11643: F, t15707: F) -> (F, F, F, F, F, F, F) {
    let t53683 = t1651 * t11620;
    let t53690 = t4892 * t11710 * t15969;
    let t53692 = t15655 * t1062;
    let t53703 = t1647 * t11239;
    let t53704 = t53703 * t11245;
    let t53707 = t53703 * t11255;
    let t53710 = t15707 * t11643;
    (t53683, t53690, t53692, t53703, t53704, t53707, t53710)
}
