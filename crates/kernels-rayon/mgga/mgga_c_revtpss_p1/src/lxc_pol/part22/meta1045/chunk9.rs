//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3668/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3668(t58145: f64, t58147: f64, t68470: f64, t68473: f64, t68476: f64, t68479: f64, t68481: f64, t68484: f64, t68486: f64, t68488: f64, t68490: f64, t68493: f64, t68495: f64, t68497: f64) -> f64 {
    let t69296 = -0.6618234375e1_f64 * t68470 + 0.264729375e1_f64 * t68473 + 0.2366859375e0_f64 * t68476 - 0.157790625e0_f64 * t68479 - 0.3529725e1_f64 * t68481 - 0.3529725e1_f64 * t68484 - 0.17648625e1_f64 * t68486 - 0.157790625e0_f64 * t68488 + 0.6311625e0_f64 * t68490 + 0.6311625e0_f64 * t68493 + 0.31558125e0_f64 * t68495 + 0.264729375e1_f64 * t68497 + 0.4630888888888888889e0_f64 * t58145 - 0.13892666666666666667e0_f64 * t58147;
    t69296
}
