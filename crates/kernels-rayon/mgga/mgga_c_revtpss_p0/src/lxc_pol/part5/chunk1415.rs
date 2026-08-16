//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1415/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1415(t6781: f64, t9593: f64, t5537: f64, t5591: f64, t13643: f64, t1448: f64, t22205: f64, t22206: f64, t22207: f64, t22208: f64, t22209: f64, t22211: f64, t5536: f64, t5541: f64, t9421: f64, t9427: f64, t9429: f64, t9514: f64, t9517: f64, t9521: f64, t9546: f64, t9569: f64, t9574: f64, t9577: f64, t9588: f64) -> f64 {
    let t22475 = t6781 * t9593;
    let t22479 = t5537 * t5591;
    let t22482 = 2.0_f64 * t1448 * t22475 * t5541 + 12.0_f64 * t22479 * t5536 - t13643 + t22205 + t22206 + t22207 + t22208 + t22209 - t22211 + t9421 - t9427 + t9429 + t9514 - t9517 - t9521 + t9546 + t9569 - t9574 - t9577 - t9588;
    t22482
}
