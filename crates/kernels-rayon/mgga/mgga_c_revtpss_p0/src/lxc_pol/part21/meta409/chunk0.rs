//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1876/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1876(t13107: f64, t489: f64, t1269: f64, t3601: f64, t3769: f64, t1248: f64, t1287: f64, t3727: f64, t3584: f64, t3759: f64, t11239: f64, t1243: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13108 = t489 * t13107;
    let t13111 = t1269 * t3601;
    let t13112 = t13111 * t3769;
    let t13118 = t3727 * t1248 * t1287;
    let t13121 = t3759 * t3584;
    let t13126 = t11239 * t1243;
    (t13108, t13111, t13112, t13118, t13121, t13126)
}
