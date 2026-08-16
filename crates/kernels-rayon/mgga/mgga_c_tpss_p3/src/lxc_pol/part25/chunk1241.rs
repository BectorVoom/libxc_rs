//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1241/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1241(t20690: f64, t645: f64, t117: f64, t20319: f64, t1279: f64, t1281: f64, t1668: f64, t1670: f64, t1851: f64, t1853: f64, t20660: f64, t20679: f64, t20682: f64, t20685: f64, t4549: f64, t4556: f64, t4559: f64, t547: f64, t548: f64, t5947: f64, t5954: f64, t5957: f64, t6446: f64, t6452: f64, t6455: f64) -> (f64, f64, f64) {
    let t20691 = t20690 * t645;
    let t20694 = t117 * t20319;
    let t20697 = 6.0_f64 * t1279 * t6452 + 3.0_f64 * t1279 * t6455 + 3.0_f64 * t1281 * t6446 + 6.0_f64 * t1668 * t5954 + 3.0_f64 * t1668 * t5957 + 3.0_f64 * t1670 * t5947 + 6.0_f64 * t1851 * t4556 + 3.0_f64 * t1851 * t4559 + 3.0_f64 * t1853 * t4549 + t20660 * t548 + 6.0_f64 * t20679 * t547 + 6.0_f64 * t20682 * t547 + 6.0_f64 * t20685 * t547 + 6.0_f64 * t20691 * t547 + 3.0_f64 * t20694 * t547;
    (t20691, t20694, t20697)
}
