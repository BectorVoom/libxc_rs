//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1012/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1012(t1568: f64, t3140: f64, t8477: f64, t1892: f64, t1501: f64, t1936: f64, t11120: f64, t11239: f64, t3268: f64, t4147: f64, t8594: f64, t8598: f64, t9593: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34074 = t1568 * t3140;
    let t34075 = t8477 * t34074;
    let t34230 = t1892 * t3140;
    let t34231 = t8477 * t34230;
    let t34258 = t1501 * t1936;
    let t36865 = t11239 * t11120;
    let t36870 = t11239 * t3268;
    let t36970 = t4147 * t8594;
    let t37110 = t9593 * t8598;
    (t34074, t34075, t34230, t34231, t34258, t36865, t36870, t36970, t37110)
}
