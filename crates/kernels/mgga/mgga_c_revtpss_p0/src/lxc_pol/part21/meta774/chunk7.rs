//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2757/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2757<F: Float>(t10811: F, t14919: F, t14904: F, t14923: F, t241: F, t40322: F, t820: F, t10665: F, t40325: F, t2659: F, t2783: F, t816: F) -> (F, F, F, F, F) {
    let t50752 = t10811 * t14919;
    let t50754 = t14923 * t14904;
    let t50757 = t820 * t40322 * t241;
    let t50758 = t40325 * t10665;
    let t50768 = t816 * t2659 * t2783;
    (t50752, t50754, t50757, t50758, t50768)
}
