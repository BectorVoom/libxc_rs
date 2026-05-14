//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 822/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk822<F: Float>(t14563: F, t2798: F, t1568: F, t2783: F, t786: F, t2435: F, t4519: F, t1558: F, t2723: F, t1531: F, t37: F, t124: F, t136: F, t243: F, t10815: F, t1561: F) -> (F, F, F, F, F, F, F, F) {
    let t14564 = t2798 * t14563;
    let t14567 = t2783 * t1568;
    let t14568 = t786 * t14567;
    let t14581 = t2435 * t4519;
    let t14586 = t1558 * t2723;
    let t14613 = t37 * t1531;
    let t14671 = t124 * t1558;
    let t14685 = t243 * t136;
    let t14712 = t10815 * t1561;
    (t14564, t14568, t14581, t14586, t14613, t14671, t14685, t14712)
}
