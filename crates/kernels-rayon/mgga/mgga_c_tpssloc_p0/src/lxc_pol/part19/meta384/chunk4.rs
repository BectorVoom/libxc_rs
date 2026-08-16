//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1439/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1439(t423: f64, t44342: f64, t44355: f64, t3330: f64, t3355: f64, t427: f64, t1129: f64, t11310: f64, t11311: f64, t11345: f64, t11350: f64, t11352: f64, t11366: f64, t1137: f64, t1138: f64, t11410: f64, t11421: f64, t1156: f64, t3327: f64, t3334: f64, t3352: f64, t3359: f64, t3360: f64, t3376: f64, t3378: f64, t3403: f64, t436: f64, t43679: f64, t43692: f64, t43951: f64, t44142: f64, t44168: f64, t44202: f64, t44205: f64, t44211: f64, t44214: f64, t44220: f64, t44223: f64, t44243: f64, t44258: f64, t44274: f64, t44289: f64, t44295: f64, t44300: f64, t44314: f64, t44327: f64) -> (f64, f64) {
    let t44358 = 0.621814e-1_f64 * (t44342 + t44355) * t423;
    let t44361 = t427 / t3355 / t3330;
    let t44366 = -0.35089341735807877242e1_f64 * t3376 * t44168 * t1156 - 0.70178683471615754484e1_f64 * t44202 * t3378 - 0.4155806185363551302e3_f64 * t44205 * t11366 + 0.6233709278045326953e3_f64 * t11310 * t43679 * t3403 - 12.0_f64 * t44211 * t3334 - 0.77193501593724168322e3_f64 * t44214 * t11421 + 0.11579025239058625248e4_f64 * t11350 * t44142 * t3359 + 0.4101607543286562663e4_f64 * t44220 * t11311 + 0.91082604192152556044e5_f64 * t44223 * t43679 * t43692 + 4.0_f64 * t3327 * t11345 + 1.0_f64 * t1129 * (t44243 + t44258 + t44274 + t44289) * t1137 + 4.0_f64 * t44295 * t1138 + 6.0_f64 * t11410 * t3352 + 0.1929837539843104208e3_f64 * t44300 * t3360 - 0.310907e-1_f64 * (t44314 + t44327) * t436 + t44358 - 0.24828486201251232145e5_f64 * t44361 * t44142 * t11352 - 0.19751673498613801407e-1_f64 * t43951;
    (t44358, t44366)
}
