//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3804/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3804(t1298: f64, t5501: f64, t18134: f64, t5023: f64, t68700: f64, t68703: f64, t68707: f64, t68709: f64, t68711: f64, t68714: f64, t68716: f64, t68718: f64, t68723: f64, t68725: f64, t68727: f64, t68730: f64, t68733: f64) -> f64 {
    let t73262 = t5501 * t1298;
    let t73266 = 8.0_f64 * t18134 * t5023 * t73262 - t68700 - t68703 + t68707 - t68709 + t68711 + t68714 - t68716 - t68718 - t68723 + t68725 + t68727 + t68730 + t68733;
    t73266
}
