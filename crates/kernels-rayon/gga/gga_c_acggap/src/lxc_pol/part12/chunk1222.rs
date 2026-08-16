//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1222/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1222(t2138: f64, t2147: f64, t2394: f64, t879: f64, t33524: f64, t639: f64, t1221: f64, t159: f64, t2146: f64, t2385: f64, t32315: f64, t32324: f64, t32329: f64, t33547: f64, t33976: f64, t36794: f64, t36808: f64, t36809: f64, t36811: f64, t38001: f64, t463: f64, t616: f64, t619: f64, t7931: f64, t8004: f64, t8306: f64, t8400: f64, t9413: f64) -> f64 {
    let t38008 = t2138 * t2147 * t2394 * t879;
    let t38010 = t33524 * t639;
    let t38013 = 0.34694512752820797848e1_f64 * t32315 - 0.26020884564615598386e1_f64 * t2146 * t8004 * t2385 * t1221 + t32324 + 0.52041769129231196772e1_f64 * t36794 + 0.4336814094102599731e0_f64 * t8400 * t8306 * t33976 - 0.52041769129231196772e1_f64 * t2146 * t8004 * t9413 * t463 - 0.17347256376410398924e1_f64 * t7931 * t8306 * t33547 - t36808 - 0.26020884564615598386e1_f64 * t36809 - 0.8673628188205199462e0_f64 * t36811 - 0.4336814094102599731e0_f64 * t616 * t619 * t159 * t38001 - 0.17347256376410398924e1_f64 * t38008 - 0.8673628188205199462e0_f64 * t38010 - 0.17347256376410398924e1_f64 * t32329;
    t38013
}
