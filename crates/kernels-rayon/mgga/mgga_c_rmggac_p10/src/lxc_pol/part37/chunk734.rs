//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 734/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk734(t14551: f64, t7508: f64, t68735: f64, t235: f64, t29837: f64, t698: f64, t2046: f64, t2050: f64, t2232: f64, t31: f64, t68757: f64, t68791: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t70948 = t7508 * t14551;
    let t71005 = 0.54934029498967360725e-3_f64 * t68735;
    let t71007 = t235 * t29837 * t698;
    let t71021 = t2046 * t2050 * t2232 * t31;
    let t71033 = 0.34547904762044099522e0_f64 * t68757;
    let t71042 = 0.86737941314158990616e-4_f64 * t68791;
    (t70948, t71005, t71007, t71021, t71033, t71042)
}
