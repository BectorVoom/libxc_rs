//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2908/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2908(t52664: f64, t52677: f64, t52690: f64, t52702: f64, t52716: f64, t52729: f64, t52743: f64, t52756: f64, t915: f64, t935: f64, t51973: f64, t41361: f64, t41363: f64, t41369: f64, t41549: f64, t51849: f64, t51853: f64, t51858: f64, t51863: f64, t51867: f64, t51871: f64, t51875: f64, t51961: f64, t51965: f64, t51967: f64, t51971: f64, t51978: f64, t52028: f64, t52031: f64, t52033: f64) -> (f64, f64) {
    let t52762 = 1.0_f64 * t915 * (t52664 + t52677 + t52690 + t52702 + t52716 + t52729 + t52743 + t52756) * t935;
    let t52774 = 0.23744444444444444444e-1_f64 * t51973;
    let t52782 = 0.4274e0_f64 * t51849 - 0.11872222222222222222e-1_f64 * t51853 - 0.52765432098765432099e-1_f64 * t51858 + 0.10685e0_f64 * t51863 + 0.10685e0_f64 * t51867 + 0.35616666666666666666e-1_f64 * t51871 - 0.42739999999999999999e0_f64 * t51875 + t41549 + 0.21369999999999999999e0_f64 * t51961 - 0.59361111111111111111e-1_f64 * t51965 + 0.17808333333333333333e-1_f64 * t51967 - 0.17808333333333333333e-1_f64 * t51971 - t52774 + 0.18467901234567901234e-1_f64 * t51978 + 0.55403703703703703702e-1_f64 * t41361 + 0.47488888888888888887e-1_f64 * t41363 - 0.23744444444444444444e-1_f64 * t41369 + 0.2137e0_f64 * t52028 + 0.23744444444444444444e0_f64 * t52031 + 0.10685e0_f64 * t52033;
    (t52762, t52782)
}
