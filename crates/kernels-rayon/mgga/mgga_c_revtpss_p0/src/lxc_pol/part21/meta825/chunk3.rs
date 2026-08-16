//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3076/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3076(t43858: f64, t43865: f64, t43883: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t56212: f64, t56214: f64, t56216: f64, t56221: f64, t56226: f64, t56229: f64, t56230: f64, t56234: f64, t56236: f64, t56248: f64, t56252: f64, t56256: f64) -> f64 {
    let t56258 = 0.61805555555555555556e-2_f64 * t56212 + 0.37083333333333333334e-1_f64 * t56214 - 0.10300925925925925926e-1_f64 * t56216 + 0.30902777777777777778e-1_f64 * t56221 + 0.55625000000000000001e-1_f64 * t56226 + t56229 - 0.92708333333333333334e-2_f64 * t56230 + 0.92708333333333333333e-2_f64 * t56234 - 0.96141975308641975309e-2_f64 * t56236 - 0.34336419753086419753e-2_f64 * t43858 - 0.82407407407407407408e-2_f64 * t43865 + 0.12361111111111111111e-1_f64 * t43883 - 0.28842592592592592593e-1_f64 * t43888 + 0.12361111111111111111e-1_f64 * t43890 + 0.24722222222222222222e-1_f64 * t43892 - 0.18541666666666666667e-1_f64 * t43894 - 0.30902777777777777778e-2_f64 * t43896 + 0.30902777777777777778e-1_f64 * t56248 + 0.166875e0_f64 * t56252 - 0.11125e0_f64 * t56256;
    t56258
}
