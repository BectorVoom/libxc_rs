//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2019/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2019(t21119: f64, t6688: f64, t3720: f64, t20266: f64, t5312: f64, t17475: f64, t20293: f64, t20318: f64, t5308: f64, t20310: f64, t20306: f64, t1260: f64, t6601: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21120 = t6688 * t21119;
    let t21121 = t3720 * t21120;
    let t21126 = t5312 * t20266;
    let t21129 = t17475 * t20293;
    let t21134 = t5308 * t20318;
    let t21137 = t5308 * t20310;
    let t21140 = t5308 * t20306;
    let t21143 = t6601 * t1260;
    (t21120, t21121, t21126, t21129, t21134, t21137, t21140, t21143)
}
