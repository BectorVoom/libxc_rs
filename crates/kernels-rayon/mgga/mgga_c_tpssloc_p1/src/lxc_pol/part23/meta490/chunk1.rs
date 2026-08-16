//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1499/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1499(t28: f64, t1302: f64, t19618: f64, t20390: f64, t3711: f64, t39877: f64, t5178: f64, t5966: f64, t77953: f64, t79873: f64, t79878: f64, t79970: f64, t1297: f64, t1390: f64, t1845: f64, t193: f64, t20077: f64, t20356: f64, t3701: f64, t3918: f64, t39604: f64, t39606: f64, t39608: f64, t39615: f64, t39635: f64, t39655: f64, t533: f64, t6347: f64, t79942: f64, t79946: f64, t79947: f64, t79952: f64, t79953: f64, t79954: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t79982 = piecewise3(t29, 0.0_f64, -56.0_f64 / 81.0_f64 * t39877 * t79873 + 16.0_f64 / 9.0_f64 * t19618 * t5966 - 2.0_f64 / 3.0_f64 * t3711 * t79878 - 8.0_f64 / 9.0_f64 * t5178 * t20390 + 2.0_f64 / 3.0_f64 * t1302 * t77953);
    let t79984 = t79970 / 2.0_f64 + t79982 / 2.0_f64;
    let t79988 = 24.0_f64 * t1390 * t1845 * t193 * t20356 - 3.0_f64 * t193 * t3701 * t533 * t79947 + 3.0_f64 * t1297 * t193 * t79984 - 18.0_f64 * t20077 * t3918 * t6347 + t39604 + t39606 + t39608 + t39615 - t39635 - t39655 + t79942 - t79946 + t79952 + t79953 + t79954;
    (t79984, t79988)
}
