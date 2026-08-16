//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1708/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1708(t1011: f64, t1012: f64, t11859: f64, t3117: f64, t3155: f64, t42508: f64, t6271: f64, t6299: f64, t67015: f64, t67186: f64, t67195: f64, t67206: f64, t79811: f64, t79818: f64, t79874: f64, t79881: f64, t79892: f64, t79938: f64, t79944: f64, t79946: f64, t87145: f64) -> f64 {
    let t89306 = t79811 / 54.0_f64 + 0.17149607247227894789e-2_f64 * t79818 + 0.57165357490759649296e-3_f64 * t67015 - 7.0_f64 / 54.0_f64 * t1011 * t1012 * t42508 * t87145 - 0.17149607247227894789e-2_f64 * t79874 - t79881 / 27.0_f64 + 0.28582678745379824648e-3_f64 * t67186 + 0.57165357490759649296e-3_f64 * t67195 + 0.34299214494455789578e-2_f64 * t79892 - 0.57165357490759649296e-3_f64 * t67206 - 0.51448821741683684368e-2_f64 * t11859 * t3117 * t6271 * t3155 * t6299 + 0.22866142996303859719e-2_f64 * t79938 - t79944 / 36.0_f64 - 0.17149607247227894789e-2_f64 * t79946;
    t89306
}
