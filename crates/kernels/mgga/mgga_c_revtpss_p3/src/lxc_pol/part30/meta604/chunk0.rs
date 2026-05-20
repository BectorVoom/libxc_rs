//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2065/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2065<F: Float>(t12995: F, t26824: F, t12963: F, t7613: F, t12975: F, t2138: F, t12984: F, t12851: F, t2134: F, t3567: F, t8945: F, t26894: F, t29199: F) -> (F, F, F, F, F, F, F) {
    let t97279 = t26824 * t12995;
    let t97281 = t7613 * t12963;
    let t97283 = t12975 * t2138;
    let t97288 = t7613 * t12984;
    let t97296 = F::new(5.0) / F::new(1296.0) * t2134 * t12851;
    let t97304 = t3567 * t8945;
    let t97308 = t26894 * t29199;
    (t97279, t97281, t97283, t97288, t97296, t97304, t97308)
}
