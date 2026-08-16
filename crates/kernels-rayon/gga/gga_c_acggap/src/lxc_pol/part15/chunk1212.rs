//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1212/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1212(t30265: f64, t34068: f64, t34076: f64, t38976: f64, t38978: f64, t38982: f64, t38986: f64, t38990: f64, t38994: f64, t38996: f64, t39000: f64, t39002: f64, t39005: f64, t39009: f64, t39013: f64, t39017: f64, t39021: f64) -> f64 {
    let t41441 = -0.62896184579208304137e-2_f64 * t38976 + 0.37737710747524982482e-2_f64 * t38978 - 0.75475421495049964966e-2_f64 * t38982 - 0.41930789719472202758e-3_f64 * t30265 - 0.17149607247227894789e-2_f64 * t34068 + t38986 / 48.0_f64 - 0.85748036236139473944e-3_f64 * t38990 + 0.85748036236139473944e-3_f64 * t38994 + 0.13719685797782315831e-1_f64 * t38996 - 0.15724046144802076034e-2_f64 * t39000 - 11.0_f64 / 48.0_f64 * t39002 - 0.916875e-1_f64 * t39005 - 0.916875e-1_f64 * t39009 - 0.916875e-1_f64 * t39013 - 0.4584375e-1_f64 * t39017 - 0.4584375e-1_f64 * t39021 + t34076;
    t41441
}
