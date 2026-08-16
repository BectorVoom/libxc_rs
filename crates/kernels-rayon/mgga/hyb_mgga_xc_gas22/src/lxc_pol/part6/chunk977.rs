//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 977/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk977(t132: f64, t9011: f64, t1238: f64, t6975: f64, t2460: f64, t3: f64, t1793: f64, t675: f64, t2002: f64, t2028: f64, t3463: f64, t3466: f64, t461: f64, t937: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t133 = t132 <= zeta_threshold;
    let t9012 = 0.18541666666666666667e-1_f64 * t9011;
    let t9013 = t6975 * t1238;
    let t9016 = t2460 * t3;
    let t9017 = t1793 * t675;
    let t9027 = piecewise3(t133, 0.0_f64, -28.0_f64 / 27.0_f64 * t9013 * t2028 - 16.0_f64 / 9.0_f64 * t9016 * t9017 + 4.0_f64 / 9.0_f64 * t3463 * t2002 + 2.0_f64 / 3.0_f64 * t937 * t1793 - 2.0_f64 * t3466 * t461);
    (t9012, t9013, t9017, t9027)
}
