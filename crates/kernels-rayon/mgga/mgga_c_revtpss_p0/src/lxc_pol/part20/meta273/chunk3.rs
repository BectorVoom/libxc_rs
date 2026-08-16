//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1127/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1127(t11788: f64, t366: f64, t1053: f64, t3223: f64, t3215: f64, t3224: f64, t1011: f64, t1028: f64, t11753: f64, t11756: f64, t11759: f64, t11763: f64, t11767: f64, t11774: f64, t11776: f64, t11779: f64, t11783: f64, t3208: f64, t3211: f64, t3220: f64, t3238: f64, t3241: f64) -> (f64, f64, f64) {
    let t11789 = t11788 * t366;
    let t11792 = t3223 * t1053;
    let t11795 = t3224 * t3215;
    let t11799 = t11753 / 288.0_f64 + t11756 / 216.0_f64 + t1011 * t11759 / 288.0_f64 - t11763 / 144.0_f64 + t1011 * t11767 / 48.0_f64 + t3241 * t3238 / 18.0_f64 - 0.85748036236139473944e-3_f64 * t11774 * t11776 - 0.21722835846488666732e-1_f64 * t11779 * t1028 - 0.64311027177104605458e-3_f64 * t11783 * t1028 - 0.64311027177104605458e-3_f64 * t3224 * t3220 + 0.12862205435420921092e-2_f64 * t11789 * t3208 + 0.68598428988911579154e-2_f64 * t11792 * t1028 - 0.85748036236139473944e-3_f64 * t11795 + 0.34299214494455789577e-2_f64 * t3211 * t3220;
    (t11789, t11792, t11799)
}
