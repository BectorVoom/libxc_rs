//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2735/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2735(t3718: f64, t44546: f64, t6689: f64, t1222: f64, t17240: f64, t20318: f64, t1263: f64, t372: f64, t6622: f64) -> (f64, f64, f64) {
    let t71294 = t3718 * t44546 * t6689;
    let t71297 = t1222 * t17240 * t20318;
    let t71300 = t372 * t1263 * t6622;
    (t71294, t71297, t71300)
}
