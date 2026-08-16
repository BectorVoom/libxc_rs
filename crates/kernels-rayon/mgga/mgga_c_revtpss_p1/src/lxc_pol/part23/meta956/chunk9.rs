//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3200/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3200(t21213: f64, t5369: f64, t59186: f64, t71550: f64, t71552: f64, t71571: f64, t71582: f64, t71598: f64, t71630: f64, t71687: f64, t71710: f64, t71718: f64) -> f64 {
    let t84049 = 0.85748036236139473944e-3_f64 * t71550 + 0.85748036236139473944e-3_f64 * t71552 + t71571 / 36.0_f64 + t71582 / 108.0_f64 - 11.0_f64 / 108.0_f64 * t21213 * t5369 - 0.57165357490759649295e-3_f64 * t71598 + t59186 - 0.85748036236139473944e-3_f64 * t71630 + 0.57165357490759649295e-3_f64 * t71687 - 0.45732285992607719436e-2_f64 * t71710 - t71718 / 81.0_f64;
    t84049
}
