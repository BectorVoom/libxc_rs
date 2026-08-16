//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2399/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2399(t11240: f64, t3144: f64, t42646: f64, t11239: f64, t989: f64, t11629: f64, t11874: f64, t16048: f64, t12046: f64, t15905: f64, t994: f64, t1011: f64, t1016: f64, t2438: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42648 = t11240 * t3144 * t42646;
    let t42668 = t989 * t11239;
    let t42669 = t42668 * t11629;
    let t42675 = t11874 * t16048;
    let t42690 = t994 * t12046 * t15905;
    let t42716 = t1011 * t2438 * t1016;
    (t42648, t42668, t42669, t42675, t42690, t42716)
}
