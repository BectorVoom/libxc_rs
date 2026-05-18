//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1354/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1354<F: Float>(t104708: F, t104905: F, t112350: F, t112364: F, t112380: F, t112397: F, t136: F, t1782: F, t22699: F, t24817: F, t24821: F, t24827: F, t24831: F, t29020: F, t29089: F, t343: F, t464: F, t6625: F, t6659: F, t6663: F, t6690: F, t7607: F) -> F {
    let t116214 = -F::new(0.68598428988911579154e-2) * t29020 * t6625 + F::new(0.13719685797782315831e-1) * t104708 * t6690 - F::new(0.17149607247227894789e-2) * t112364 - F::new(11.0) / F::new(108.0) * t112350 * t1782 + t7607 * t24831 / F::new(36.0) + t29089 * t6659 / F::new(36.0) + t29089 * t6663 / F::new(18.0) - t7607 * t24817 / F::new(288.0) - t7607 * t24821 / F::new(48.0) - F::new(0.17149607247227894789e-2) * t112380 + t112397 / F::new(216.0) - F::new(77.0) / F::new(162.0) * t22699 * t343 * t136 * t464 - F::new(7.0) / F::new(648.0) * t7607 * t24827 - F::new(0.28582678745379824648e-3) * t104905;
    t116214
}
