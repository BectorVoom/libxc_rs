//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2961/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2961(t1087: f64, t43065: f64, t3105: f64, t4857: f64, t1012: f64, t43222: f64, t16190: f64, t3173: f64, t1011: f64, t11714: f64, t15144: f64, t15830: f64, t16012: f64, t16095: f64, t16096: f64, t16196: f64, t16223: f64, t3092: f64, t3101: f64, t3106: f64, t3130: f64, t4803: f64, t4919: f64, t51851: f64, t51856: f64, t51925: f64, t51930: f64) -> (f64, f64) {
    let t53923 = t1087 * t43065;
    let t53926 = t4857 * t3105;
    let t53944 = t1012 * t43222;
    let t53948 = t16190 * t3173;
    let t53954 = -0.76220476654346199061e-2_f64 * t53923 * t16223 + 0.45732285992607719436e-2_f64 * t53926 * t3130 + 0.45732285992607719436e-2_f64 * t15830 * t3101 + 0.91464571985215438873e-2_f64 * t11714 * t4803 + 0.45732285992607719436e-2_f64 * t3106 * t16196 + t1011 * t4919 * t51925 / 72.0_f64 + 7.0_f64 / 216.0_f64 * t1011 * t16012 * t51930 + t1011 * t4919 * t51851 / 216.0_f64 + 35.0_f64 / 972.0_f64 * t1011 * t53944 * t51856 - 0.45732285992607719436e-2_f64 * t53948 + 0.17149607247227894789e-2_f64 * t16095 * t3092 * t15144 * t16096;
    (t53923, t53954)
}
