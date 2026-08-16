//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 680/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk680(t1016: f64, t140: f64, t1011: f64, t1015: f64, t2258: f64, t1012: f64, t271: f64, t905: f64, t2852: f64, t2251: f64, t1017: f64, t1025: f64, t1028: f64, t1068: f64, t3188: f64, t3191: f64, t3194: f64, t3197: f64, t3203: f64, t3205: f64, t3208: f64, t3211: f64, t3216: f64, t3220: f64, t3224: f64, t3231: f64, t3234: f64, t3238: f64, t3241: f64, t375: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3244 = t140 * t1016;
    let t3245 = t1011 * t3244;
    let t3247 = t1015 * t2258;
    let t3248 = t1012 * t3247;
    let t3252 = 1.0_f64 / t271 / t905;
    let t3253 = t3252 * t2852;
    let t3254 = t3253 * t2251;
    let t3255 = t1012 * t3254;
    let t3258 = 0.28582678745379824648e-3_f64 * t3188 * t1068 - 0.22866142996303859718e-2_f64 * t3191 * t375 + 0.28582678745379824648e-3_f64 * t3194 + 0.21437009059034868486e-3_f64 * t3197 * t375 - t3203 + 0.42874018118069736972e-3_f64 * t3205 * t3208 + 0.22866142996303859718e-2_f64 * t3211 * t1028 - 0.28582678745379824648e-3_f64 * t3216 - 0.21437009059034868486e-3_f64 * t1025 * t3220 - 0.42874018118069736972e-3_f64 * t3224 * t1028 + 0.72409452821628889107e-2_f64 * t3231 * t375 - 0.15244095330869239812e-2_f64 * t3234 - t1011 * t3238 / 144.0_f64 - t3241 * t1017 / 54.0_f64 + t3245 / 432.0_f64 + t1011 * t3248 / 288.0_f64 + t1011 * t3255 / 216.0_f64;
    (t3244, t3245, t3247, t3252, t3254, t3258)
}
