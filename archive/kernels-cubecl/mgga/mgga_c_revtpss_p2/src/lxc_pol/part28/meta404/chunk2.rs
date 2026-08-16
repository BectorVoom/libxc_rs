//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1521/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1521<F: Float>(t1548: F, t2394: F, t800: F, t2689: F, t4372: F, t4354: F, t9775: F, t14468: F, t828: F, t855: F, t221: F, t2675: F, t4343: F) -> (F, F, F, F, F) {
    let t14843 = t800 * t1548 * t2394;
    let t14846 = t2689 * t4372;
    let t14850 = t9775 * t4354;
    let t14853 = t855 * t828 * t14468;
    let t14857 = t2675 * t221 * t4343;
    (t14843, t14846, t14850, t14853, t14857)
}
