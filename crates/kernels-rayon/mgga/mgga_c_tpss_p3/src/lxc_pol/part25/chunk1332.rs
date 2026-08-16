//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1332/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1332(t114: f64, t71158: f64, t117: f64, t1279: f64, t1281: f64, t16052: f64, t16067: f64, t16076: f64, t1668: f64, t1670: f64, t1851: f64, t1853: f64, t20660: f64, t20678: f64, t20679: f64, t20685: f64, t20691: f64, t20694: f64, t21958: f64, t21975: f64, t21981: f64, t4549: f64, t4556: f64, t4637: f64, t4674: f64, t547: f64, t5815: f64, t6446: f64, t6452: f64, t6455: f64) -> (f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t71159 = piecewise3(t115, 0.0_f64, t71158);
    let t71181 = 3.0_f64 * t117 * t547 * t71159 + 6.0_f64 * t20678 * t4674 * t547 + 6.0_f64 * t4637 * t547 * t5815 + 12.0_f64 * t1279 * t21975 + 3.0_f64 * t1279 * t21981 + 3.0_f64 * t1281 * t21958 + 3.0_f64 * t16052 * t1853 + 12.0_f64 * t16067 * t1851 + 3.0_f64 * t16076 * t1851 + 12.0_f64 * t1668 * t20679 + 12.0_f64 * t1668 * t20685 + 12.0_f64 * t1668 * t20691 + 6.0_f64 * t1668 * t20694 + 6.0_f64 * t1670 * t20660 + 12.0_f64 * t4549 * t6452 + 6.0_f64 * t4549 * t6455 + 12.0_f64 * t4556 * t6446;
    (t71159, t71181)
}
