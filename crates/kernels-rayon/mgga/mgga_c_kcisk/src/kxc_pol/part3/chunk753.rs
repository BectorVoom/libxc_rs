//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 753/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk753(t1049: f64, t695: f64, t10399: f64, t10441: f64, t10449: f64, t11495: f64, t11613: f64, t11615: f64, t11623: f64, t11626: f64, t11630: f64, t11633: f64, t1809: f64, t1850: f64, t5089: f64, t5168: f64) -> f64 {
    let t11634 = t1049 * t695;
    let t11635 = 0.62154466893555682512e-3_f64 * t11634;
    let t11636 = 0.11955719325063177623e-1_f64 * t1809 * t10449 - 0.93231700340333523768e-3_f64 * t11613 + 0.31077233446777841256e-3_f64 * t11615 - 0.5179538907796306876e-4_f64 * t1850 * t10449 - 0.71734315950379065738e-1_f64 * t5089 * t10399 + 0.46615850170166761884e-3_f64 * t5168 * t10399 + 0.71734315950379065738e-1_f64 * t11623 - 0.93231700340333523768e-3_f64 * t11626 + 0.71734315950379065738e-1_f64 * t11495 * t10441 - 0.62154466893555682512e-3_f64 * t11630 * t10441 + t11633 - t11635;
    t11636
}
