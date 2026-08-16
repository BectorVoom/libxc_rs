//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1243/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1243(t13293: f64, t13299: f64, t6403: f64, t8401: f64, t1095: f64, t1795: f64, t384: f64, t398: f64, t879: f64, t1017: f64, t1180: f64, t1181: f64, t13286: f64, t13287: f64, t1532: f64, t15386: f64, t17445: f64, t17450: f64, t17454: f64, t17468: f64, t17480: f64, t175: f64, t1753: f64, t17656: f64, t1854: f64, t20124: f64, t3196: f64, t3403: f64, t397: f64, t4313: f64, t5984: f64, t922: f64) -> f64 {
    let t22809 = t13293 * t13299 * t8401 * t6403;
    let t22818 = t384 * t398 * t1095 * t1795 * t879;
    let t22826 = -0.51448821741683684367e-2_f64 * t1180 * t1181 * t4313 * t1753 * t1017 + 0.17149607247227894789e-2_f64 * t17445 + 0.85748036236139473944e-3_f64 * t17450 - 0.17149607247227894789e-2_f64 * t17454 - 0.17149607247227894789e-1_f64 * t3403 * t1181 * t1532 * t1753 * t922 - 0.51448821741683684366e-2_f64 * t17656 * t15386 * t8401 * t5984 - 0.17149607247227894789e-2_f64 * t22809 - 0.13719685797782315831e-1_f64 * t13286 * t13287 * t1854 * t3196 + 0.42874018118069736972e-3_f64 * t22818 + 0.34299214494455789578e-2_f64 * t17468 - 0.68598428988911579156e-2_f64 * t17480 - 0.21437009059034868486e-3_f64 * t397 * t398 * t175 * t20124;
    t22826
}
