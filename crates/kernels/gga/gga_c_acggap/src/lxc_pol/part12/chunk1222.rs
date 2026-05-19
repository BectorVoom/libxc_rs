//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1222/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1222<F: Float>(t2138: F, t2147: F, t2394: F, t879: F, t33524: F, t639: F, t1221: F, t159: F, t2146: F, t2385: F, t32315: F, t32324: F, t32329: F, t33547: F, t33976: F, t36794: F, t36808: F, t36809: F, t36811: F, t38001: F, t463: F, t616: F, t619: F, t7931: F, t8004: F, t8306: F, t8400: F, t9413: F) -> F {
    let t38008 = t2138 * t2147 * t2394 * t879;
    let t38010 = t33524 * t639;
    let t38013 = F::cast_from(0.34694512752820797848e1_f64) * t32315 - F::cast_from(0.26020884564615598386e1_f64) * t2146 * t8004 * t2385 * t1221 + t32324 + F::cast_from(0.52041769129231196772e1_f64) * t36794 + F::cast_from(0.4336814094102599731e0_f64) * t8400 * t8306 * t33976 - F::cast_from(0.52041769129231196772e1_f64) * t2146 * t8004 * t9413 * t463 - F::cast_from(0.17347256376410398924e1_f64) * t7931 * t8306 * t33547 - t36808 - F::cast_from(0.26020884564615598386e1_f64) * t36809 - F::cast_from(0.8673628188205199462e0_f64) * t36811 - F::cast_from(0.4336814094102599731e0_f64) * t616 * t619 * t159 * t38001 - F::cast_from(0.17347256376410398924e1_f64) * t38008 - F::cast_from(0.8673628188205199462e0_f64) * t38010 - F::cast_from(0.17347256376410398924e1_f64) * t32329;
    t38013
}
