//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 209/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk209(t841: f64, t846: f64, t848: f64, t812: f64, t833: f64, t834: f64, t836: f64, t839: f64) -> (f64, f64) {
    let t849 = t841 * t846 * t848;
    let t852 = t812 + t833 - 0.18311555036753159941e-3_f64 * t834 * t836 - 0.58482233974552040708e0_f64 * t839 * t849;
    (t849, t852)
}
