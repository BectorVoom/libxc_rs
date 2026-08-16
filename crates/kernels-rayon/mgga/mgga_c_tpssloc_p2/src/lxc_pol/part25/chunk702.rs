//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 702/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk702(t533: f64, t7216: f64, t1390: f64, t2095: f64, t6999: f64, t113: f64, t1266: f64, t1393: f64, t1983: f64, t2036: f64, t2040: f64, t2075: f64, t2079: f64, t2096: f64, t2314: f64, t4034: f64, t510: f64, t574: f64, t650: f64, t652: f64, t672: f64, t6876: f64, t7040: f64, t7042: f64, t7050: f64, t7057: f64, t7061: f64, t7156: f64, t7166: f64, t7171: f64) -> (f64, f64, f64, f64) {
    let t7217 = t533 * t7216;
    let t7218 = t7217 * t1390;
    let t7220 = t2095 * t6999;
    let t7222 = -t113 * t7156 - t1266 * t2036 + t1393 * t2079 + 3.0_f64 * t1983 * t7171 + t1983 * t7218 - t1983 * t7220 - 2.0_f64 * t2040 * t2314 - 2.0_f64 * t2040 * t4034 - t2075 * t650 + t2096 * t6876 - t510 * t7040 + t574 * t7166 - 2.0_f64 * t652 * t7050 - 2.0_f64 * t652 * t7057 - 2.0_f64 * t652 * t7061 - 2.0_f64 * t672 * t7042;
    (t7217, t7218, t7220, t7222)
}
