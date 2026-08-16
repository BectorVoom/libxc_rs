//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3095/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3095(t17351: f64, t17354: f64, t56756: f64, t3588: f64, t3611: f64, t1042: f64, t1121: f64, t12273: f64, t1250: f64, t12809: f64, t12822: f64, t12945: f64, t13065: f64, t13081: f64, t17353: f64, t17412: f64, t17763: f64, t21275: f64, t3568: f64, t3620: f64, t3640: f64, t3711: f64, t3720: f64, t44521: f64, t5277: f64, t5346: f64, t5381: f64, t5391: f64, t56713: f64, t56718: f64, t56720: f64, t56727: f64, t56728: f64, t56731: f64, t56734: f64, t56740: f64, t56742: f64, t606: f64) -> f64 {
    let t56758 = t17351 * t56756 * t17354;
    let t56760 = t3611 * t3588;
    let t56765 = 0.57165357490759649295e-3_f64 * t56713 - 0.85748036236139473944e-3_f64 * t21275 * t13081 + 0.95275595817932748827e-3_f64 * t56718 + 0.95275595817932748827e-3_f64 * t56720 + 0.85748036236139473944e-3_f64 * t3711 * t1042 * t5277 * t12273 - t56727 - 0.30488190661738479624e-2_f64 * t56728 + 0.21437009059034868486e-3_f64 * t56731 * t13065 - 0.11433071498151929859e-2_f64 * t56734 + 0.71456696863449561621e-3_f64 * t5381 * t12945 - t56740 - 0.95275595817932748826e-4_f64 * t56742 + 0.22866142996303859718e-2_f64 * t17412 * t3640 + 0.7622047665434619906e-3_f64 * t5391 * t12822 + 0.7145669686344956162e-3_f64 * t17763 * t3620 - 0.85748036236139473944e-3_f64 * t44521 * t17353 * t1250 * t3568 * t1121 * t606 + 0.57165357490759649295e-3_f64 * t56758 + 0.64311027177104605458e-3_f64 * t12809 * t3720 * t5346 * t56760;
    t56765
}
