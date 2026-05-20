//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2730/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2730<F: Float>(t12772: F, t21160: F, t3625: F, t11249: F, t6622: F, t12832: F, t20926: F, t15904: F, t17394: F, t13127: F, t3682: F, t6667: F) -> (F, F, F, F, F, F) {
    let t70857 = t3625 * t12772 * t21160;
    let t70890 = t6622 * t11249;
    let t70914 = t12832 * t20926;
    let t70916 = t17394 * t15904;
    let t70917 = t13127 * t70916;
    let t70942 = t6667 * t3682;
    (t70857, t70890, t70914, t70916, t70917, t70942)
}
