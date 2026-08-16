//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3505/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3505(t1043: f64, t4181: f64, t1058: f64, t19869: f64, t3201: f64, t6318: f64, t1011: f64, t15987: f64, t18926: f64, t18930: f64, t16226: f64, t19957: f64, t19960: f64, t19963: f64, t3230: f64, t3241: f64, t375: f64, t43174: f64, t4915: f64, t53320: f64, t53328: f64, t53832: f64, t53859: f64, t53875: f64, t55209: f64, t60927: f64, t6317: f64, t63313: f64) -> (f64, f64) {
    let t66128 = t4181 * t1043;
    let t66139 = t19869 * t1058;
    let t66141 = t6318 * t3201;
    let t66155 = t1011 * t15987 * t18926;
    let t66158 = t1011 * t15987 * t18930;
    let t66161 = -0.22866142996303859718e-2_f64 * t16226 * t55209 * t43174 * t66128 + t53320 * t53328 * t60927 / 12.0_f64 + 0.72409452821628889107e-2_f64 * t6317 * t3230 * t375 - 0.15244095330869239812e-2_f64 * t66139 - 0.47637797908966374413e-4_f64 * t66141 + 11.0_f64 / 486.0_f64 * t53832 - 0.28582678745379824648e-3_f64 * t53859 - 14.0_f64 / 243.0_f64 * t3241 * t19957 - t1011 * t4915 * t63313 / 12.0_f64 - t3241 * t19960 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t3241 * t19963 + t66155 / 72.0_f64 - t66158 / 108.0_f64 + 0.19055119163586549765e-3_f64 * t53875;
    (t66128, t66161)
}
