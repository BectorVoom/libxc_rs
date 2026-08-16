//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 689/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk689(t10733: f64, t10748: f64, t1664: f64, t1645: f64, t1643: f64, t4740: f64, t573: f64, t4743: f64, t586: f64, t10560: f64, t10568: f64, t10570: f64, t10572: f64, t10574: f64, t10576: f64, t10579: f64, t10582: f64, t10587: f64, t10590: f64, t10595: f64, t10598: f64) -> (f64, f64, f64) {
    let t10749 = t10733 + t10748;
    let t10750 = t10749 * t1664;
    let t10752 = 1.0_f64 * t1645 * t10750;
    let t10754 = 1.0_f64 / t4740 / t1643;
    let t10755 = t573 * t10754;
    let t10757 = 1.0_f64 / t4743 / t586;
    let t10758 = t10560 * t10757;
    let t10760 = 0.51725014705706168417e3_f64 * t10755 * t10758;
    let t10761 = 0.28842592592592592592e-1_f64 * t10568;
    let t10772 = -t10761 - 0.12361111111111111111e-1_f64 * t10570 + 0.61805555555555555556e-2_f64 * t10572 - 0.18541666666666666667e-1_f64 * t10574 + 0.92708333333333333334e-2_f64 * t10576 - 0.10300925925925925926e-1_f64 * t10579 + 0.37083333333333333333e-1_f64 * t10582 - 0.18541666666666666666e-1_f64 * t10587 - 0.55625000000000000001e-1_f64 * t10590 + 0.55625000000000000001e-1_f64 * t10595 - 0.92708333333333333333e-2_f64 * t10598;
    (t10752, t10760, t10772)
}
