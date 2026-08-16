//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1701/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1701(t6244: f64, t6305: f64, t1011: f64, t1063: f64, t1066: f64, t11632: f64, t11774: f64, t1469: f64, t15689: f64, t15696: f64, t16012: f64, t16226: f64, t23907: f64, t247: f64, t3117: f64, t3155: f64, t42472: f64, t42621: f64, t43050: f64, t4893: f64, t4915: f64, t6266: f64, t6267: f64, t66777: f64, t67052: f64, t79219: f64, t79233: f64, t79253: f64, t79450: f64, t88087: f64, t88095: f64, t88102: f64, t88106: f64, t88116: f64, t88794: f64) -> (f64, f64) {
    let t89084 = t6244 * t6305;
    let t89094 = 7.0_f64 / 108.0_f64 * t1011 * t16012 * t88116 + t1011 * t4915 * t88087 / 8.0_f64 - t1011 * t4915 * t88095 / 36.0_f64 - 0.17149607247227894789e-2_f64 * t15689 * t66777 * t4893 * t6266 - 0.17149607247227894789e-2_f64 * t11774 * t67052 * t6267 - 0.17149607247227894789e-2_f64 * t11774 * t15696 * t23907 + 0.34299214494455789578e-2_f64 * t16226 * t66777 * t3155 * t79450 * t1469 + 0.22866142996303859718e-2_f64 * t79219 + 0.14291339372689912324e-3_f64 * t1063 * t247 * t1066 * t88106 + 0.23289590088828005269e-2_f64 * t1063 * t247 * t42472 * t88102 - 0.17149607247227894789e-2_f64 * t79233 + 0.51448821741683684368e-2_f64 * t43050 * t3117 * t89084 * t3155 - 0.17149607247227894789e-2_f64 * t79253 - 0.51448821741683684368e-2_f64 * t42621 * t3117 * t88794 * t11632;
    (t89084, t89094)
}
