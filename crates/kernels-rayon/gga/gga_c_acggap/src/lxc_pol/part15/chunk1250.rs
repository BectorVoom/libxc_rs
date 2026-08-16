//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1250/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1250(t32833: f64, t32834: f64, t32839: f64, t35713: f64, t37658: f64, t37661: f64, t37663: f64, t37665: f64, t37672: f64, t37675: f64, t40105: f64, t40107: f64, t40109: f64, t40111: f64, t40114: f64, t40118: f64, t40121: f64, t40123: f64) -> f64 {
    let t41948 = -t37658 + 0.34299214494455789578e-1_f64 * t40105 + t37661 - 0.34299214494455789578e-1_f64 * t40107 - 0.34299214494455789578e-2_f64 * t40109 - 0.34299214494455789578e-2_f64 * t40111 - t37663 - t37665 + 0.85748036236139473944e-3_f64 * t40114 + t37672 + 0.31448092289604152068e-2_f64 * t40118 - t32833 - t32834 - t37675 - t35713 - t32839 - 7.0_f64 / 24.0_f64 * t40121 - t40123 / 24.0_f64;
    t41948
}
