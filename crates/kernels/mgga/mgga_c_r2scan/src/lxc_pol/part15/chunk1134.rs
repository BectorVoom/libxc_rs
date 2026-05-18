//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1134/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1134<F: Float>(t1058: F, t1060: F, t2201: F, t7290: F, t10894: F, t2630: F, t10784: F, t3613: F, t5103: F, t10844: F, t11760: F, t3308: F, t37965: F, t7538: F) -> (F, F, F, F, F) {
    let t39599 = t2201 * t1058 * t1060 * t7290;
    let t39601 = t10894 * t2630;
    let t39602 = F::new(0.54878743191129263322e-2) * t39601;
    let t39604 = t5103 * t3613 * t10784;
    let t39607 = t2201 * t11760 * t10844;
    let t39608 = F::new(0.46574606203128791246e-1) * t39607;
    let t39610 = t37965 * t3308 * t7538;
    (t39599, t39602, t39604, t39608, t39610)
}
