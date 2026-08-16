//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 494/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk494(t6163: f64, t6257: f64, t1168: f64, t1174: f64, t1452: f64, t1454: f64, t1455: f64, t1459: f64, t1857: f64, t1860: f64, t228: f64, t4444: f64, t458: f64, t462: f64, t5555: f64, t5558: f64, t598: f64, t6071: f64, t6073: f64, t6077: f64, t6080: f64, t6086: f64, t6093: f64, t6096: f64, t6099: f64, t6102: f64, t6105: f64) -> (f64, f64) {
    let t6258 = t6163 + t6257;
    let t6261 = t6071 * t228 + t6073 * t1455 + t1857 * t1168 / 4.0_f64 + 2.0_f64 * t598 * t6077 + t6080 * t1455 + t1860 * t1168 / 4.0_f64 + t1452 * t1459 / 2.0_f64 + t1454 * t6086 / 2.0_f64 - 5.0_f64 / 8.0_f64 * t598 * t5555 + t598 * t5558 / 2.0_f64 - 5.0_f64 / 16.0_f64 * t458 * t6093 + 45.0_f64 / 64.0_f64 * t4444 * t6096 - 5.0_f64 / 8.0_f64 * t1174 * t6099 + t458 * t6102 / 4.0_f64 - 5.0_f64 / 16.0_f64 * t1174 * t6105 + t462 * t6258 / 4.0_f64;
    (t6258, t6261)
}
