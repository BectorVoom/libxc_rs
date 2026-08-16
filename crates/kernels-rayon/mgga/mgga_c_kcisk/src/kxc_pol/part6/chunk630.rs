//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 630/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk630(t1725: f64, t8729: f64, t4911: f64, t8697: f64, t4915: f64, t7076: f64, t8684: f64, t8687: f64, t8690: f64, t2430: f64, t1746: f64, t4928: f64) -> (f64, f64, f64, f64, f64) {
    let t8730 = t8729 * t1725;
    let t8733 = t8697 * t4911;
    let t8740 = t4915 + 0.61805555555555555556e-2_f64 * t7076 - 0.61805555555555555555e-2_f64 * t8684 + 0.18541666666666666667e-1_f64 * t8687 - 0.92708333333333333333e-2_f64 * t8690;
    let t8746 = t2430 * t2430;
    let t8748 = t4928 * t8746 * t1746;
    (t8730, t8733, t8740, t8746, t8748)
}
