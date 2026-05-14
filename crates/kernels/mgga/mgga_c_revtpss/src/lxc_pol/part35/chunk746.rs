//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 746/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk746<F: Float>(t1626: F, t3011: F, t1614: F, t2967: F, t2986: F, t1596: F, t2923: F, t3090: F, t4954: F, t1646: F, t3056: F, t225: F, t366: F, t372: F, t4823: F, t1062: F, t4857: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15350 = t1626 * t3011;
    let t15406 = t1614 * t2967;
    let t15413 = t1626 * t2986;
    let t15421 = t1596 * t2923;
    let t15618 = t4954 * t3090;
    let t15669 = t1646 * t3056;
    let t15670 = t15669 * t225;
    let t15671 = t15670 * t366;
    let t15696 = t372 * t4823;
    let t15707 = t4857 * t1062;
    (t15350, t15406, t15413, t15421, t15618, t15669, t15670, t15671, t15696, t15707)
}
