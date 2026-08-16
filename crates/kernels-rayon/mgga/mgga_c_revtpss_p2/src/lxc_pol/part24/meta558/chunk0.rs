//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1669/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1669(t77804: f64, t88085: f64, t88093: f64, t88104: f64, t88108: f64, t88114: f64, t88122: f64, t88130: f64, t88220: f64, t88222: f64, t88224: f64, t88226: f64, t88229: f64, t88232: f64) -> f64 {
    let t88321 = -0.705945e1_f64 * t88220 - 0.94674375e0_f64 * t88222 + 0.1262325e1_f64 * t88224 + 0.158837625e2_f64 * t88226 - 0.27785333333333333334e0_f64 * t88229 + 0.83356e0_f64 * t88232 + 0.123954e2_f64 * t88085 + 0.309885e1_f64 * t88093 - 0.15302962962962962963e1_f64 * t88104 - 0.516475e0_f64 * t88108 + 0.68863333333333333334e1_f64 * t88114 - 0.123954e2_f64 * t88122 - 0.103295e1_f64 * t88130 - 0.166712e1_f64 * t77804;
    t88321
}
