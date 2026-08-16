//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2390/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2390<F: Float>(t10705: F, t10716: F, t10697: F, t136: F, t10627: F, t221: F, t2674: F, t2452: F, t9720: F, t225: F, t268: F, t2665: F) -> (F, F, F, F, F, F, F) {
    let t40681 = t10716 * t10705;
    let t40683 = t10697 * t136;
    let t40686 = t2674 * t40683 * t221 * t10627;
    let t40688 = t9720 * t2452;
    let t40689 = t40688 * t225;
    let t40690 = t268 * t40689;
    let t40691 = t40690 * t2665;
    (t40681, t40683, t40686, t40688, t40689, t40690, t40691)
}
