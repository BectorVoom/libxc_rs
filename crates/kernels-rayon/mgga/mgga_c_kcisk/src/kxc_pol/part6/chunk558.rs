//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 558/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk558(t1191: f64, t7785: f64, t1172: f64, t3679: f64, t7753: f64, t3677: f64, t3683: f64, t5668: f64, t7738: f64, t7742: f64, t7746: f64, t334: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7786 = t7785 * t1191;
    let t7788 = 1.0_f64 * t1172 * t7786;
    let t7789 = t7753 * t3679;
    let t7791 = 0.16081824322151104822e2_f64 * t3677 * t7789;
    let t7796 = t3683 + 0.61805555555555555556e-2_f64 * t5668 - 0.61805555555555555555e-2_f64 * t7738 + 0.18541666666666666667e-1_f64 * t7742 - 0.92708333333333333333e-2_f64 * t7746;
    let t7797 = t7796 * t334;
    (t7786, t7788, t7789, t7791, t7796, t7797)
}
