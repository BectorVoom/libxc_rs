//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 337/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk337<F: Float>(t1633: F, t973: F, t1598: F, t1612: F, t1614: F, t1622: F, t1627: F, t300: F, t311: F, t946: F, t965: F, t964: F, t981: F, t1594: F, t986: F, t341: F) -> (F, F, F, F, F, F, F) {
    let t1634 = t1633 * t973;
    let t1638 = t300 * (-0.310907e-1 * t1614 * t311 + 1.0 * t946 * t1622 + t1598 - t1612 - 0.19751673498613801407e-1 * t1627 + 0.5848223622634646207e0 * t965 * t1634);
    let t1640 = 0.19751673498613801407e-1 * t300 * t1627;
    let t1642 = t964 * t1633 * t973;
    let t1644 = 0.5848223622634646207e0 * t981 * t1642;
    let t1646 = -t986 - 0.83333333333333333333e-2 * t1594;
    let t1647 = t1646 * t341;
    (t1634, t1638, t1640, t1642, t1644, t1646, t1647)
}
