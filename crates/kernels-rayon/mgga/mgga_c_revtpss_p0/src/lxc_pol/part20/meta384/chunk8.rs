//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1410/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1410(t11075: f64, t2403: f64, t2430: f64, t40093: f64, t40095: f64, t40099: f64, t40103: f64, t40106: f64, t40109: f64, t40111: f64, t40115: f64, t40117: f64, t40120: f64, t40122: f64, t40126: f64) -> f64 {
    let t41174 = 18.0_f64 * t11075 * t2403 * t2430 - t40093 + t40095 + t40099 + t40103 + t40106 - t40109 + t40111 - t40115 + t40117 + t40120 + t40122 - t40126;
    t41174
}
