//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 241/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk241(t206: f64, t220: f64, t62: f64, t691: f64, t20: f64, t212: f64, t870: f64, t209: f64, t208: f64, t214: f64, t217: f64, t221: f64, t712: f64, t750: f64, t777: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t210 = 0.0_f64 < t206;
    let t872 = t220 * t220;
    let t873 = 1.0_f64 / t872;
    let t874 = t206 * t873;
    let t875 = t62 * t691;
    let t876 = t875 * t20;
    let t879 = t212 * t212;
    let t880 = 1.0_f64 / t879;
    let t882 = piecewise3(t210, t870, -t870);
    let t884 = t209 * t880 * t882;
    let t887 = -7.0_f64 / 288.0_f64 * t876 * t214 - t208 * t884 / 96.0_f64;
    let t888 = 1.0_f64 / t217;
    let t889 = t887 * t888;
    let t895 = t870 * t221 - 0.66725e-1_f64 * t874 * t889 - 0.92858888888888888886e-2_f64 * t712 + 0.69644166666666666665e-2_f64 * t750 - 0.69644166666666666665e-2_f64 * t777;
    (t872, t873, t874, t876, t879, t880, t882, t884, t887, t888, t889, t895)
}
