//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3026/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3026<F: Float>(t241: F, t820: F, t849: F, t14900: F, t14923: F, t10811: F, t14914: F, t14788: F, t10886: F, t14652: F, t808: F, t14746: F, t2703: F) -> (F, F, F, F, F, F) {
    let t50957 = t820 * t849 * t241;
    let t50966 = t14923 * t14900;
    let t50968 = t10811 * t14914;
    let t50974 = t10811 * t14788;
    let t50977 = t10886 * t808 * t14652;
    let t50982 = t2703 * t14746;
    (t50957, t50966, t50968, t50974, t50977, t50982)
}
