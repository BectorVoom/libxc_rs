//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 453/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk453<F: Float>(t1191: F, t3671: F, t1172: F, t1170: F, t305: F, t320: F, t3640: F, t3571: F, t3573: F, t3577: F, t3581: F, t3585: F, t334: F, t1197: F, t45: F, t1202: F, t330: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3672 = t3671 * t1191;
    let t3674 = 1.0 * t1172 * t3672;
    let t3675 = t1170 * t1170;
    let t3676 = 1.0 / t3675;
    let t3677 = t305 * t3676;
    let t3678 = t320 * t320;
    let t3679 = 1.0 / t3678;
    let t3680 = t3640 * t3679;
    let t3682 = 0.16081824322151104822e2 * t3677 * t3680;
    let t3683 = 0.12361111111111111111e-1 * t3571;
    let t3688 = t3683 + 0.61805555555555555556e-2 * t3573 - 0.61805555555555555555e-2 * t3577 + 0.18541666666666666667e-1 * t3581 - 0.92708333333333333333e-2 * t3585;
    let t3689 = t3688 * t334;
    let t3692 = t45 * t1197;
    let t3695 = t1202 * t330;
    (t3672, t3674, t3675, t3676, t3677, t3678, t3679, t3680, t3682, t3688, t3689, t3692, t3695)
}
