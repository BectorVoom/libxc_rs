//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3748/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3748(t1263: f64, t372: f64, t6622: f64, t17605: f64, t17635: f64, t17646: f64, t17654: f64, t17657: f64, t17781: f64, t21306: f64, t21310: f64, t3631: f64, t3674: f64, t3720: f64, t44422: f64, t44797: f64, t5340: f64, t5341: f64, t57348: f64, t6611: f64, t70311: f64, t71275: f64, t71278: f64, t71280: f64, t71294: f64, t71297: f64) -> (f64, f64) {
    let t71300 = t372 * t1263 * t6622;
    let t71304 = 0.30488190661738479624e-2_f64 * t17605 * t17635 + 0.30488190661738479624e-2_f64 * t17605 * t17646 + 0.30488190661738479624e-2_f64 * t71275 * t3631 + 0.30488190661738479624e-2_f64 * t71278 + 0.14481890564325777821e-1_f64 * t71280 * t3674 + 0.42874018118069736972e-3_f64 * t44422 * t6611 - t44797 - 0.85748036236139473944e-3_f64 * t21306 * t17781 + 0.85748036236139473944e-3_f64 * t5340 * t3720 * t70311 * t5341 - 0.11433071498151929859e-2_f64 * t57348 * t21310 + 0.95275595817932748827e-4_f64 * t71294 - t71297 / 216.0_f64 - 0.57165357490759649296e-3_f64 * t17654 * t71300 * t17657;
    (t71300, t71304)
}
