//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 961/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk961(t1256: f64, t8185: f64, t1238: f64, t1791: f64, t26827: f64, t26855: f64, t26863: f64, t29047: f64, t29055: f64, t29062: f64, t29065: f64, t29069: f64, t29072: f64, t29074: f64, t484: f64, t5320: f64, t7613: f64) -> f64 {
    let t29077 = t8185 * t1256;
    let t29079 = t29047 * t29055 / 216.0_f64 - 0.42874018118069736972e-3_f64 * t26827 * t1791 - 0.42874018118069736972e-3_f64 * t7613 * t5320 + 0.22866142996303859718e-2_f64 * t29062 * t1238 - 0.28582678745379824648e-3_f64 * t29065 - 0.19055119163586549765e-3_f64 * t26855 + 0.28582678745379824648e-3_f64 * t26863 - 0.22866142996303859718e-2_f64 * t29069 * t484 + 0.28582678745379824648e-3_f64 * t29072 + 0.42874018118069736972e-3_f64 * t29074 * t484 - 0.15244095330869239812e-2_f64 * t29077;
    t29079
}
