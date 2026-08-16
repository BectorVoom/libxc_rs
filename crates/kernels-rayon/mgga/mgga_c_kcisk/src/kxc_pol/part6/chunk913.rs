//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 913/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk913(t29211: f64, t29226: f64, t1725: f64, t10928: f64, t29195: f64, t10934: f64, t17382: f64, t23460: f64, t23472: f64, t23481: f64, t29082: f64, t29085: f64, t29088: f64, t29091: f64, t29094: f64, t29097: f64) -> (f64, f64, f64) {
    let t29227 = t29211 + t29226;
    let t29228 = t29227 * t1725;
    let t29231 = t29195 * t10928;
    let t29244 = -t10934 - 0.12361111111111111111e-1_f64 * t17382 + 0.61805555555555555556e-2_f64 * t23460 - 0.18541666666666666667e-1_f64 * t23472 + 0.92708333333333333334e-2_f64 * t23481 - 0.10300925925925925926e-1_f64 * t29082 + 0.37083333333333333333e-1_f64 * t29085 - 0.18541666666666666666e-1_f64 * t29088 - 0.55625000000000000001e-1_f64 * t29091 + 0.55625000000000000001e-1_f64 * t29094 - 0.92708333333333333333e-2_f64 * t29097;
    (t29228, t29231, t29244)
}
