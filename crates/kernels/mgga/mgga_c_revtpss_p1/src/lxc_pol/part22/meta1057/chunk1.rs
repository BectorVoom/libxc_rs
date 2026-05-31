//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3748/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3748<F: Float>(t1263: F, t372: F, t6622: F, t17605: F, t17635: F, t17646: F, t17654: F, t17657: F, t17781: F, t21306: F, t21310: F, t3631: F, t3674: F, t3720: F, t44422: F, t44797: F, t5340: F, t5341: F, t57348: F, t6611: F, t70311: F, t71275: F, t71278: F, t71280: F, t71294: F, t71297: F) -> (F, F) {
    let t71300 = t372 * t1263 * t6622;
    let t71304 = F::cast_from(0.30488190661738479624e-2_f64) * t17605 * t17635 + F::cast_from(0.30488190661738479624e-2_f64) * t17605 * t17646 + F::cast_from(0.30488190661738479624e-2_f64) * t71275 * t3631 + F::cast_from(0.30488190661738479624e-2_f64) * t71278 + F::cast_from(0.14481890564325777821e-1_f64) * t71280 * t3674 + F::cast_from(0.42874018118069736972e-3_f64) * t44422 * t6611 - t44797 - F::cast_from(0.85748036236139473944e-3_f64) * t21306 * t17781 + F::cast_from(0.85748036236139473944e-3_f64) * t5340 * t3720 * t70311 * t5341 - F::cast_from(0.11433071498151929859e-2_f64) * t57348 * t21310 + F::cast_from(0.95275595817932748827e-4_f64) * t71294 - t71297 / F::cast_from(216.0_f64) - F::cast_from(0.57165357490759649296e-3_f64) * t17654 * t71300 * t17657;
    (t71300, t71304)
}
