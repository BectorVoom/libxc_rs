//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 977/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk977(t118: f64, t1986: f64, t495: f64, t571: f64, t7717: f64, t2001: f64, t498: f64, t7720: f64, t1618: f64, t1600: f64, t11905: f64, t2028: f64, t2604: f64, t36402: f64, t36416: f64, t36418: f64, t36448: f64, t36453: f64, t40679: f64, t40681: f64, t40683: f64, t40685: f64, t40688: f64, t40690: f64, t8994: f64) -> f64 {
    let t40694 = t1986 * t118 * t571 * t495;
    let t40695 = t7717 * t40694;
    let t40699 = t2001 * t118 * t571 * t498;
    let t40700 = t7720 * t40699;
    let t40702 = t1986 * t1618;
    let t40703 = t7720 * t40702;
    let t40705 = t1986 * t1600;
    let t40706 = t7720 * t40705;
    let t40714 = 0.20001418546446583934e0_f64 * t36402 + 0.54549323308490683458e-1_f64 * t36416 - 0.72732431077987577944e-1_f64 * t36418 - 0.41382249896261788303e-4_f64 * t40679 - 0.33105799917009430643e-4_f64 * t40681 - 0.25538759935978703638e-4_f64 * t40683 - 0.5987120850931904282e-1_f64 * t40685 - 0.2993560425465952141e-1_f64 * t40688 + 0.2993560425465952141e-1_f64 * t40690 + 0.1064114997332445985e-4_f64 * t40695 + 0.85129199786595678796e-5_f64 * t40700 - 0.25538759935978703638e-4_f64 * t40703 - 0.25538759935978703638e-4_f64 * t40706 - 0.11974241701863808564e0_f64 * t11905 * t2028 - 0.59590439850616975158e-4_f64 * t36448 - 0.19863479950205658386e-4_f64 * t36453 - 0.11974241701863808564e0_f64 * t2604 * t8994;
    t40714
}
