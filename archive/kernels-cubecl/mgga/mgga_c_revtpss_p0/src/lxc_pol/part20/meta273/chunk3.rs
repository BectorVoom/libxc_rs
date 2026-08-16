//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1127/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1127<F: Float>(t11788: F, t366: F, t1053: F, t3223: F, t3215: F, t3224: F, t1011: F, t1028: F, t11753: F, t11756: F, t11759: F, t11763: F, t11767: F, t11774: F, t11776: F, t11779: F, t11783: F, t3208: F, t3211: F, t3220: F, t3238: F, t3241: F) -> (F, F, F) {
    let t11789 = t11788 * t366;
    let t11792 = t3223 * t1053;
    let t11795 = t3224 * t3215;
    let t11799 = t11753 / F::cast_from(288.0_f64) + t11756 / F::cast_from(216.0_f64) + t1011 * t11759 / F::cast_from(288.0_f64) - t11763 / F::cast_from(144.0_f64) + t1011 * t11767 / F::cast_from(48.0_f64) + t3241 * t3238 / F::cast_from(18.0_f64) - F::cast_from(0.85748036236139473944e-3_f64) * t11774 * t11776 - F::cast_from(0.21722835846488666732e-1_f64) * t11779 * t1028 - F::cast_from(0.64311027177104605458e-3_f64) * t11783 * t1028 - F::cast_from(0.64311027177104605458e-3_f64) * t3224 * t3220 + F::cast_from(0.12862205435420921092e-2_f64) * t11789 * t3208 + F::cast_from(0.68598428988911579154e-2_f64) * t11792 * t1028 - F::cast_from(0.85748036236139473944e-3_f64) * t11795 + F::cast_from(0.34299214494455789577e-2_f64) * t3211 * t3220;
    (t11789, t11792, t11799)
}
