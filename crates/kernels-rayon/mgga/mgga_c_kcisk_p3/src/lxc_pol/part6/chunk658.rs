//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 658/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk658(t1987: f64, t2396: f64, t240: f64, t7517: f64, t8546: f64, t8548: f64, t8552: f64, t8576: f64, t8579: f64, t8585: f64, t8592: f64, t8609: f64, t8613: f64, t9143: f64) -> f64 {
    let t9155 = -t8546 + t8548 - t8552 + t8576 + t8579 + t240 * t9143 + 0.19751789702565206229e-1_f64 * t240 * t8585 - 0.11696446794910408142e1_f64 * t7517 * t2396 + 0.11696446794910408142e1_f64 * t1987 * t8592 - 0.58482233974552040708e0_f64 * t1987 * t8609 - 0.17315755899375863299e2_f64 * t1987 * t8613;
    t9155
}
