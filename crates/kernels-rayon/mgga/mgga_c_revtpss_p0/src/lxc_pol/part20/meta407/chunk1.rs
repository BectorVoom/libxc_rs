//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1507/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1507(t11977: f64, t3173: f64, t12009: f64, t12013: f64, t11916: f64, t11999: f64, t3043: f64, t3140: f64, t3149: f64, t11239: f64, t989: f64, t11629: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42658 = t11977 * t3173;
    let t42660 = t12013 * t12009;
    let t42662 = t11999 * t11916;
    let t42664 = t3043 * t3140;
    let t42665 = t42664 * t3149;
    let t42668 = t989 * t11239;
    let t42669 = t42668 * t11629;
    (t42658, t42660, t42662, t42664, t42665, t42668, t42669)
}
