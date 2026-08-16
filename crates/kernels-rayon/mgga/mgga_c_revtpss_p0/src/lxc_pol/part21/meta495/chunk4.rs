//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2092/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2092(t15670: f64, t366: f64, t3106: f64, t4817: f64, t1025: f64, t1028: f64, t11644: f64, t11649: f64, t11783: f64, t15651: f64, t15656: f64, t15662: f64, t15668: f64, t1665: f64, t3208: f64, t3211: f64, t3220: f64, t3224: f64, t4854: f64, t4858: f64) -> (f64, f64) {
    let t15671 = t15670 * t366;
    let t15675 = 0.10162730220579493208e-2_f64 * t3106 * t4817;
    let t15676 = -0.19055119163586549765e-3_f64 * t11644 + 0.14291339372689912324e-3_f64 * t11649 - 0.21437009059034868486e-3_f64 * t11783 * t1665 - 0.42874018118069736972e-3_f64 * t3224 * t4854 - 0.21437009059034868486e-3_f64 * t1025 * t15651 - 0.42874018118069736972e-3_f64 * t15656 * t1028 - 0.21437009059034868486e-3_f64 * t4858 * t3220 - t15662 + 0.22866142996303859718e-2_f64 * t3211 * t4854 - t15668 + 0.42874018118069736972e-3_f64 * t15671 * t3208 - t15675;
    (t15671, t15676)
}
