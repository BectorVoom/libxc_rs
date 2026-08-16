//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1625/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1625(t11133: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11147: f64, t11153: f64, t11158: f64, t11162: f64, t11167: f64, t11171: f64) -> f64 {
    let t11173 = -t11133 - 0.19755555555555555556e-1_f64 * t11134 + 0.9877777777777777778e-2_f64 * t11136 - 0.29633333333333333334e-1_f64 * t11138 + 0.14816666666666666667e-1_f64 * t11140 - 0.16462962962962962963e-1_f64 * t11147 + 0.59266666666666666668e-1_f64 * t11153 - 0.29633333333333333334e-1_f64 * t11158 - 0.88900000000000000002e-1_f64 * t11162 + 0.88900000000000000002e-1_f64 * t11167 - 0.14816666666666666667e-1_f64 * t11171;
    t11173
}
