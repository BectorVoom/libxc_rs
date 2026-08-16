//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3059/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3059(t18893: f64, t3359: f64, t11303: f64, t11350: f64, t1136: f64, t11415: f64, t11420: f64, t15117: f64, t15146: f64, t15159: f64, t15165: f64, t15168: f64, t15172: f64, t1683: f64, t18631: f64, t18634: f64, t18637: f64, t18640: f64, t18643: f64, t18644: f64, t18650: f64, t18894: f64, t3332: f64, t3333: f64, t3351: f64, t3357: f64, t44214: f64, t44361: f64, t4824: f64, t51427: f64, t51599: f64, t51604: f64, t6037: f64, t6053: f64, t6056: f64) -> f64 {
    let t63502 = t18893 * t3359;
    let t63506 = -0.38596750796862084161e3_f64 * t51427 * t15159 + 0.12865583598954028054e3_f64 * t51599 * t4824 + 0.12865583598954028054e3_f64 * t15146 * t15165 + 0.64327917994770140268e2_f64 * t15146 * t15168 + 0.4138081033541872024e4_f64 * t51604 * t15172 + 12.0_f64 * t11415 * t18631 + 6.0_f64 * t3357 * t6037 * t3351 + 0.11579025239058625248e4_f64 * t11350 * t6056 * t3333 - 8.0_f64 * t11303 * t18634 - 4.0_f64 * t3332 * t1683 * t15117 - 0.38596750796862084162e3_f64 * t44214 * t18637 - 0.19298375398431042081e3_f64 * t11420 * t6056 * t3351 - 0.24828486201251232145e5_f64 * t44361 * t18650 * t3333 - 4.0_f64 * t11303 * t18640 - 4.0_f64 * t3332 * t18894 * t1136 - 2.0_f64 * t3332 * t6053 * t3351 - 0.19298375398431042081e3_f64 * t11420 * t18643 * t3333 + 0.64327917994770140268e2_f64 * t11415 * t18644 + 0.64327917994770140268e2_f64 * t3357 * t63502 * t1136;
    t63506
}
