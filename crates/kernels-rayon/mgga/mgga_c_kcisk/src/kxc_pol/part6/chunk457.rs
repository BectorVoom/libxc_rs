//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 457/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk457(t1170: f64, t305: f64, t320: f64, t3571: f64, t1202: f64, t330: f64, t3657: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3675 = t1170 * t1170;
    let t3676 = 1.0_f64 / t3675;
    let t3677 = t305 * t3676;
    let t3678 = t320 * t320;
    let t3679 = 1.0_f64 / t3678;
    let t3683 = 0.12361111111111111111e-1_f64 * t3571;
    let t3695 = t1202 * t330;
    let t3696 = 1.0_f64 / t3695;
    let t3704 = 0.40256666666666666667e0_f64 * t3571;
    let t3711 = 0.137975e0_f64 * t3657;
    let t3721 = t1202 * t1202;
    let t3722 = 1.0_f64 / t3721;
    (t3675, t3676, t3677, t3678, t3679, t3683, t3696, t3704, t3711, t3721, t3722)
}
