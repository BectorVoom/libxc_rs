//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1379/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1379(t71259: f64, t71303: f64, t71343: f64, t71878: f64, t72593: f64, t72637: f64, t72682: f64, t72721: f64, t1279: f64, t1338: f64, t13546: f64, t16064: f64, t16073: f64, t1668: f64, t1851: f64, t19040: f64, t20682: f64, t20690: f64, t21972: f64, t21978: f64, t25315: f64, t3537: f64, t4559: f64, t4674: f64, t547: f64, t5470: f64, t5474: f64, t5477: f64, t548: f64, t5947: f64, t5953: f64, t5954: f64, t5957: f64, t6446: f64, t645: f64, t67816: f64, t71184: f64, t71212: f64, param_d: f64) -> (f64, f64) {
    let t72724 = t71259 + t71303 + t71343 + t71878 + t72593 + t72637 + t72682 + t72721;
    let t72733 = 6.0_f64 * t5947 * t5474 + 12.0_f64 * t547 * t71184 * t1338 + 12.0_f64 * t547 * t67816 * t1338 + 12.0_f64 * t547 * t20690 * t3537 + 6.0_f64 * t1279 * t21972 + 3.0_f64 * t5947 * t5477 + 6.0_f64 * t6446 * t4559 + 6.0_f64 * t1851 * t16064 + 3.0_f64 * t5470 * t5957 + 6.0_f64 * t547 * t19040 * t4674 + 6.0_f64 * t547 * t5953 * t13546 + 6.0_f64 * t1851 * t16073 + 6.0_f64 * t547 * t71212 * t645 + 12.0_f64 * t547 * t25315 * t3537 + param_d * t72724 * t548 + 12.0_f64 * t1668 * t20682 + 6.0_f64 * t1279 * t21978 + 6.0_f64 * t5470 * t5954;
    (t72724, t72733)
}
