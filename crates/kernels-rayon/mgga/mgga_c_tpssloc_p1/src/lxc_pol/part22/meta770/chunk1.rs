//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2622/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2622(t15338: f64, t18427: f64, t3447: f64, t22032: f64, t3448: f64, t11570: f64, t20234: f64, t1409: f64, t15293: f64, t18416: f64, t18420: f64, t18469: f64, t18542: f64, t3449: f64, t3450: f64, t4900: f64, t4908: f64, t4919: f64, t4928: f64, t52140: f64, t71168: f64, t71172: f64, t71181: f64, t71185: f64, t73138: f64) -> (f64, f64, f64) {
    let t73199 = t3447 * t15338 * t18427;
    let t73201 = t3448 * t22032;
    let t73225 = t11570 * t20234;
    let t73252 = 0.83333333333333333331e-3_f64 * t3447 * t18416 * t18542 + 0.16666666666666666666e-2_f64 * t3447 * t18416 * t15293 - 0.49999999999999999998e-2_f64 * t3447 * t4908 * t71168 + 0.16666666666666666666e-2_f64 * t3447 * t3449 * t73225 - 0.66666666666666666664e-2_f64 * t3447 * t4908 * t71172 + 0.11111111111111111111e-2_f64 * t3447 * t4900 * t71181 + 0.11111111111111111111e-2_f64 * t3447 * t4900 * t71185 - 0.11111111111111111111e-2_f64 * t3447 * t52140 * t18469 + 0.66666666666666666665e-2_f64 * t3447 * t4900 * t73138 + 0.16666666666666666666e-2_f64 * t3447 * t4919 * t3450 * t1409 * t4928 + 0.83333333333333333331e-3_f64 * t3447 * t18420 * t18542;
    (t73199, t73201, t73252)
}
