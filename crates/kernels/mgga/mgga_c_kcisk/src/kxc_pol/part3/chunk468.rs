//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 468/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk468<F: Float>(t1191: F, t3671: F, t1172: F, t1170: F, t305: F, t320: F, t3640: F, t3571: F, t3573: F, t3577: F, t3581: F, t3585: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3672 = t3671 * t1191;
    let t3674 = F::new(1.0) * t1172 * t3672;
    let t3675 = t1170 * t1170;
    let t3676 = F::new(1.0) / t3675;
    let t3677 = t305 * t3676;
    let t3678 = t320 * t320;
    let t3679 = F::new(1.0) / t3678;
    let t3680 = t3640 * t3679;
    let t3682 = F::cast_from(0.16081824322151104822e2_f64) * t3677 * t3680;
    let t3683 = F::cast_from(0.12361111111111111111e-1_f64) * t3571;
    let t3688 = t3683 + F::cast_from(0.61805555555555555556e-2_f64) * t3573 - F::cast_from(0.61805555555555555555e-2_f64) * t3577 + F::cast_from(0.18541666666666666667e-1_f64) * t3581 - F::cast_from(0.92708333333333333333e-2_f64) * t3585;
    (t3672, t3674, t3675, t3676, t3677, t3678, t3679, t3680, t3682, t3688)
}
