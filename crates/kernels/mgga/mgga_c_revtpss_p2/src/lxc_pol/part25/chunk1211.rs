//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1211/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1211<F: Float>(t10301: F, t607: F, t10309: F, t1927: F, t2248: F, t1926: F, t25163: F, t6973: F, t644: F, t6977: F, t25113: F, t77: F) -> (F, F, F, F, F, F) {
    let t92565 = t10301 * t607;
    let t92568 = t10309 * t607;
    let t92569 = t1927 * t2248;
    let t92570 = t1926 * t92569;
    let t92573 = t6973 * t25163;
    let t92576 = t6977 * t644;
    let t92577 = t1926 * t92576;
    let t92581 = t77 * t25113 * t644;
    (t92565, t92568, t92570, t92573, t92577, t92581)
}
