//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 411/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk411<F: Float>(t176: F, t844: F, t88: F, t194: F, t843: F, t189: F, t850: F, t851: F, t2318: F, t2321: F, t2323: F, t2327: F, t2329: F, t2331: F) -> (F, F, F, F, F, F, F, F) {
    let t2640 = t176 * t176;
    let t2641 = F::new(1.0) / t2640;
    let t2658 = t88 * t844;
    let t2662 = t843 * t194;
    let t2663 = F::new(1.0) / t2662;
    let t2664 = t189 * t2663;
    let t2665 = t850 * t850;
    let t2666 = t2665 * t851;
    let t2675 = -F::new(0.78438333333333333333e0) * t2318 + F::new(0.15687666666666666667e1) * t2321 + F::new(0.68863333333333333333e0) * t2323 + F::new(0.14025833333333333333e0) * t2327 + F::new(0.28051666666666666667e0) * t2329 + F::new(0.17365833333333333333e0) * t2331;
    (t2640, t2641, t2658, t2663, t2664, t2665, t2666, t2675)
}
