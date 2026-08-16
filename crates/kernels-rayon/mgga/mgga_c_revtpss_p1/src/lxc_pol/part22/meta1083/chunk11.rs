//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3924/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3924(t22479: f64, t47122: f64, t47124: f64, t47131: f64, t47138: f64, t47140: f64, t47142: f64, t49541: f64, t74139: f64, t74141: f64, t74142: f64, t74143: f64, t74144: f64) -> f64 {
    let t75412 = 24.0_f64 * t22479 * t49541 + t47122 + t47124 + t47131 - t47138 - t47140 + t47142 + t74139 + t74141 + t74142 - t74143 - t74144;
    t75412
}
