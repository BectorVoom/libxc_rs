//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 468/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk468(t1191: f64, t3671: f64, t1172: f64, t1170: f64, t305: f64, t320: f64, t3640: f64, t3571: f64, t3573: f64, t3577: f64, t3581: f64, t3585: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3672 = t3671 * t1191;
    let t3674 = 1.0_f64 * t1172 * t3672;
    let t3675 = t1170 * t1170;
    let t3676 = 1.0_f64 / t3675;
    let t3677 = t305 * t3676;
    let t3678 = t320 * t320;
    let t3679 = 1.0_f64 / t3678;
    let t3680 = t3640 * t3679;
    let t3682 = 0.16081824322151104822e2_f64 * t3677 * t3680;
    let t3683 = 0.12361111111111111111e-1_f64 * t3571;
    let t3688 = t3683 + 0.61805555555555555556e-2_f64 * t3573 - 0.61805555555555555555e-2_f64 * t3577 + 0.18541666666666666667e-1_f64 * t3581 - 0.92708333333333333333e-2_f64 * t3585;
    (t3672, t3674, t3675, t3676, t3677, t3678, t3679, t3680, t3682, t3688)
}
