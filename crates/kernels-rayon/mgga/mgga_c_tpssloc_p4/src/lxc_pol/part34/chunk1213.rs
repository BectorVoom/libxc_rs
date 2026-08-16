//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1213/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1213(t107802: f64, t107822: f64, t107842: f64, t107860: f64, t107268: f64, t20044: f64, t20060: f64, t20594: f64, t2085: f64, t2092: f64, t27068: f64, t29299: f64, t29311: f64, t5215: f64, t5321: f64, t539: f64, t568: f64, t6361: f64, t6440: f64, t74908: f64, t7918: f64, t7925: f64, t97664: f64) -> (f64, f64) {
    let t107862 = t107802 + t107822 + t107842 + t107860;
    let t107875 = 0.9869604401089358619e-1_f64 * t107268 + 6.0_f64 * t27068 * t6440 + 6.0_f64 * t20044 * t7925 - 3.0_f64 * t74908 * t2092 - 18.0_f64 * t5215 * t29299 + t539 * t107862 * t568 - 0.69087230807625510332e0_f64 * t97664 + 3.0_f64 * t6361 * t7918 * t568 + 6.0_f64 * t20060 * t7925 + 12.0_f64 * t5321 * t29311 + t20594 * t2085 * t568;
    (t107862, t107875)
}
