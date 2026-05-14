//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1017/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1017<F: Float>(t10894: F, t2630: F, t10784: F, t3613: F, t5103: F, t10844: F, t11760: F, t2201: F, t3308: F, t37965: F, t7538: F, t2214: F, t3293: F, t528: F, t132: F, t1567: F) -> (F, F, F, F, F, F) {
    let t39601 = t10894 * t2630;
    let t39602 = 0.54878743191129263322e-2 * t39601;
    let t39604 = t5103 * t3613 * t10784;
    let t39607 = t2201 * t11760 * t10844;
    let t39608 = 0.46574606203128791246e-1 * t39607;
    let t39610 = t37965 * t3308 * t7538;
    let t39613 = t3293 * t2214 * t528;
    let t39614 = t132 * t1567;
    (t39602, t39604, t39608, t39610, t39613, t39614)
}
