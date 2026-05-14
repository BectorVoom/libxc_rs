//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 352/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk352<F: Float>(t265: F, t393: F, t1079: F, t1695: F, t1076: F, t1647: F, t1652: F, t1680: F, t342: F, t386: F, t995: F, t1102: F, t1587: F, t1598: F, t1612: F, t1638: F, t1640: F, t1644: F, t198: F, t336: F) -> (F, F, F) {
    let t394 = t265 < t393;
    let t1696 = t1079 * t1695;
    let t1699 = 0.65854491829355115987e0 * t1647 * t386 - 0.65854491829355115987e0 * t995 * t1652 + 0.65854491829355115987e0 * t342 * t1680 - 0.65854491829355115987e0 * t1076 * t1696;
    let t1704 = piecewise3(t394, t1102 * t1699 * t198 * t336 - t1598 + t1612 + t1638 + t1640 - t1644, t1587);
    (t1696, t1699, t1704)
}
