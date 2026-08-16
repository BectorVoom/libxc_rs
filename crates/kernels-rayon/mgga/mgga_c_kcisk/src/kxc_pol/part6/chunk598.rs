//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 598/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk598(t1550: f64, t2107: f64, t240: f64, t6568: f64, t7750: f64, t7752: f64, t7756: f64, t7788: f64, t7791: f64, t7797: f64, t7804: f64, t7821: f64, t7825: f64, t8384: f64) -> f64 {
    let t8396 = -t7750 + t7752 - t7756 + t7788 + t7791 + t240 * t8384 + 0.19751789702565206229e-1_f64 * t240 * t7797 - 0.11696446794910408142e1_f64 * t6568 * t2107 + 0.11696446794910408142e1_f64 * t1550 * t7804 - 0.58482233974552040708e0_f64 * t1550 * t7821 - 0.17315755899375863299e2_f64 * t1550 * t7825;
    t8396
}
