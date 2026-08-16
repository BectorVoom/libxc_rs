//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2901/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2901(t41361: f64, t41520: f64, t51978: f64, t52337: f64, t63276: f64, t63278: f64, t77499: f64, t77503: f64, t77505: f64, t77507: f64, t77509: f64, t77515: f64, t77518: f64, t77521: f64, t77527: f64, t77531: f64, t77535: f64, t77539: f64, t77543: f64, t77547: f64) -> f64 {
    let t77549 = 0.34336419753086419753e-2_f64 * t77499 - 0.92708333333333333333e-2_f64 * t77503 + 0.30902777777777777778e-2_f64 * t77505 - 0.12361111111111111111e-1_f64 * t77507 + 0.18541666666666666667e-1_f64 * t77509 - 0.18541666666666666667e-1_f64 * t63276 + 0.61805555555555555556e-2_f64 * t63278 + t41520 + 0.11125e0_f64 * t77515 - 0.30902777777777777777e-1_f64 * t77518 - 0.166875e0_f64 * t77521 - t52337 + 0.28842592592592592592e-1_f64 * t51978 + 0.96141975308641975307e-2_f64 * t41361 - 0.18541666666666666666e-1_f64 * t77527 - 0.18541666666666666666e-1_f64 * t77531 + 0.2225e0_f64 * t77535 - 0.166875e0_f64 * t77539 + 0.55625000000000000001e-1_f64 * t77543 + 0.55625000000000000001e-1_f64 * t77547;
    t77549
}
