//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2883/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2883(t51973: f64, t41361: f64, t41363: f64, t41369: f64, t41520: f64, t51849: f64, t51853: f64, t51858: f64, t51863: f64, t51867: f64, t51871: f64, t51875: f64, t51961: f64, t51965: f64, t51967: f64, t51971: f64, t51978: f64, t52028: f64, t52031: f64, t52033: f64) -> f64 {
    let t52337 = 0.12361111111111111111e-1_f64 * t51973;
    let t52345 = 0.2225e0_f64 * t51849 - 0.61805555555555555555e-2_f64 * t51853 - 0.27469135802469135803e-1_f64 * t51858 + 0.55625000000000000001e-1_f64 * t51863 + 0.55625000000000000001e-1_f64 * t51867 + 0.18541666666666666667e-1_f64 * t51871 - 0.22249999999999999999e0_f64 * t51875 + t41520 + 0.11125e0_f64 * t51961 - 0.30902777777777777778e-1_f64 * t51965 + 0.92708333333333333334e-2_f64 * t51967 - 0.92708333333333333333e-2_f64 * t51971 - t52337 + 0.96141975308641975309e-2_f64 * t51978 + 0.28842592592592592593e-1_f64 * t41361 + 0.24722222222222222222e-1_f64 * t41363 - 0.12361111111111111111e-1_f64 * t41369 + 0.11125e0_f64 * t52028 + 0.12361111111111111111e0_f64 * t52031 + 0.55625e-1_f64 * t52033;
    t52345
}
