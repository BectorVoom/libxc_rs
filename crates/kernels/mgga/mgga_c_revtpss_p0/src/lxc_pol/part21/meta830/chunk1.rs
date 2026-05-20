//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3095/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3095<F: Float>(t17351: F, t17354: F, t56756: F, t3588: F, t3611: F, t1042: F, t1121: F, t12273: F, t1250: F, t12809: F, t12822: F, t12945: F, t13065: F, t13081: F, t17353: F, t17412: F, t17763: F, t21275: F, t3568: F, t3620: F, t3640: F, t3711: F, t3720: F, t44521: F, t5277: F, t5346: F, t5381: F, t5391: F, t56713: F, t56718: F, t56720: F, t56727: F, t56728: F, t56731: F, t56734: F, t56740: F, t56742: F, t606: F) -> F {
    let t56758 = t17351 * t56756 * t17354;
    let t56760 = t3611 * t3588;
    let t56765 = F::cast_from(0.57165357490759649295e-3_f64) * t56713 - F::cast_from(0.85748036236139473944e-3_f64) * t21275 * t13081 + F::cast_from(0.95275595817932748827e-3_f64) * t56718 + F::cast_from(0.95275595817932748827e-3_f64) * t56720 + F::cast_from(0.85748036236139473944e-3_f64) * t3711 * t1042 * t5277 * t12273 - t56727 - F::cast_from(0.30488190661738479624e-2_f64) * t56728 + F::cast_from(0.21437009059034868486e-3_f64) * t56731 * t13065 - F::cast_from(0.11433071498151929859e-2_f64) * t56734 + F::cast_from(0.71456696863449561621e-3_f64) * t5381 * t12945 - t56740 - F::cast_from(0.95275595817932748826e-4_f64) * t56742 + F::cast_from(0.22866142996303859718e-2_f64) * t17412 * t3640 + F::cast_from(0.7622047665434619906e-3_f64) * t5391 * t12822 + F::cast_from(0.7145669686344956162e-3_f64) * t17763 * t3620 - F::cast_from(0.85748036236139473944e-3_f64) * t44521 * t17353 * t1250 * t3568 * t1121 * t606 + F::cast_from(0.57165357490759649295e-3_f64) * t56758 + F::cast_from(0.64311027177104605458e-3_f64) * t12809 * t3720 * t5346 * t56760;
    t56765
}
