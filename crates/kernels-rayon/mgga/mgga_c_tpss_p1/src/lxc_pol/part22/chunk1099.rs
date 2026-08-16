//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1099/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1099(t11848: f64, t11850: f64, t11853: f64, t11896: f64, t11899: f64, t11904: f64, t11908: f64, t11913: f64, t11916: f64, t11919: f64, t11922: f64, t11925: f64, t12024: f64, t12040: f64, t12046: f64, t12064: f64, t9183: f64, t9192: f64, t9194: f64, t9196: f64, t9429: f64, t9438: f64) -> f64 {
    let t12066 = -t9429 + 0.23154444444444444444e-1_f64 * t9183 + 0.23154444444444444444e0_f64 * t9192 - 0.69463333333333333333e-1_f64 * t9194 - 0.13892666666666666667e0_f64 * t9196 - t12024 + 0.104195e0_f64 * t11848 + 0.11577222222222222222e0_f64 * t11850 - t9438 + 0.264729375e1_f64 * t11853 + t12040 - 0.34431666666666666667e0_f64 * t11896 + 0.309885e1_f64 * t11899 + 0.20659e1_f64 * t11904 + 0.103295e1_f64 * t11908 - t12046 - 0.69463333333333333334e-1_f64 * t11913 - 0.34731666666666666667e-1_f64 * t11916 - 0.20839e0_f64 * t11919 + 0.41678e0_f64 * t11922 + 0.20839e0_f64 * t11925 + t12064;
    t12066
}
