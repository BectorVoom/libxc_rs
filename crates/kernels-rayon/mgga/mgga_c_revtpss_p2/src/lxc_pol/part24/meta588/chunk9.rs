//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1845/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1845(t6781: f64, t198: f64, t22466: f64, t40067: f64, t40072: f64, t47088: f64, t47092: f64, t47096: f64, t47098: f64, t47109: f64, t47116: f64, t47118: f64, t47672: f64, t532: f64, t5536: f64, t6836: f64, t92019: f64, t92020: f64, t92021: f64, t92022: f64) -> f64 {
    let t92495 = t6781 * t6781;
    let t92500 = -6.0_f64 * t198 * t47672 * t532 * t92495 - 36.0_f64 * t22466 * t5536 * t6836 + t40067 - t40072 + t47088 + t47092 - t47096 - t47098 - t47109 + t47116 - t47118 + t92019 - t92020 - t92021 + t92022;
    t92500
}
