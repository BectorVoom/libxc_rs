//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2926/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2926(t15885: f64, t993: f64, t378: f64, t51973: f64, t41361: f64, t41363: f64, t41369: f64, t42078: f64, t51849: f64, t51853: f64, t51858: f64, t51863: f64, t51867: f64, t51871: f64, t51875: f64, t51961: f64, t51965: f64, t51967: f64, t51971: f64, t51978: f64, t52028: f64, t52031: f64, t52033: f64) -> (f64, f64, f64) {
    let t53222 = t15885 * t993;
    let t53223 = t53222 * t378;
    let t53243 = 0.19755555555555555556e-1_f64 * t51973;
    let t53251 = 0.35560000000000000001e0_f64 * t51849 - 0.9877777777777777778e-2_f64 * t51853 - 0.43901234567901234568e-1_f64 * t51858 + 0.88900000000000000002e-1_f64 * t51863 + 0.88900000000000000002e-1_f64 * t51867 + 0.29633333333333333334e-1_f64 * t51871 - 0.35560000000000000001e0_f64 * t51875 + t42078 + 0.17780000000000000001e0_f64 * t51961 - 0.49388888888888888889e-1_f64 * t51965 + 0.14816666666666666667e-1_f64 * t51967 - 0.14816666666666666667e-1_f64 * t51971 - t53243 + 0.15365432098765432099e-1_f64 * t51978 + 0.46096296296296296298e-1_f64 * t41361 + 0.39511111111111111113e-1_f64 * t41363 - 0.19755555555555555556e-1_f64 * t41369 + 0.1778e0_f64 * t52028 + 0.19755555555555555556e0_f64 * t52031 + 0.88900000000000000002e-1_f64 * t52033;
    (t53222, t53223, t53251)
}
