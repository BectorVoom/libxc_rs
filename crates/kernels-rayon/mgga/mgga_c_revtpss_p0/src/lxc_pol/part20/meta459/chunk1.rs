//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1750/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1750(t2516: f64, t9551: f64, t3863: f64, t4029: f64, t39989: f64, t40067: f64, t47082: f64, t47084: f64, t47086: f64, t47088: f64, t47090: f64, t47092: f64, t47094: f64, t47096: f64, t47098: f64) -> (f64, f64, f64) {
    let t47099 = t9551 * t2516;
    let t47100 = 0.35089341735807877242e1_f64 * t47099;
    let t47101 = t3863 * t4029;
    let t47102 = 384.0_f64 * t47101;
    let t47103 = t47082 - t47084 - t39989 - t47086 + t47088 + t47090 + t47092 + t47094 - t47096 - t47098 - t47100 - t47102 + t40067;
    (t47100, t47102, t47103)
}
