//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1441/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1441(t15376: f64, t15390: f64, t18409: f64, t18420: f64, t18427: f64, t18469: f64, t22072: f64, t22075: f64, t22090: f64, t22095: f64, t3447: f64, t4904: f64, t4919: f64, t52081: f64, t64648: f64, t73181: f64, t73201: f64, t73405: f64, t73427: f64) -> f64 {
    let t78489 = -0.22222222222222222222e-2_f64 * t3447 * t64648 * t18469 - 0.88888888888888888887e-2_f64 * t15376 * t22095 + 0.11111111111111111111e-2_f64 * t3447 * t73201 * t4904 + 0.11111111111111111111e-2_f64 * t3447 * t4919 * t73405 - 0.88888888888888888886e-2_f64 * t15376 * t22072 - 0.11111111111111111111e-2_f64 * t73427 - 0.17777777777777777777e-1_f64 * t15376 * t22075 - 0.88888888888888888886e-2_f64 * t3447 * t15390 * t73181 + 0.17777777777777777777e-1_f64 * t15376 * t22090 + 0.16666666666666666666e-2_f64 * t3447 * t18420 * t18409 + 0.33333333333333333332e-2_f64 * t3447 * t18420 * t18427 - 0.12345679012345679012e-2_f64 * t52081;
    t78489
}
