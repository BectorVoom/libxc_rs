//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 890/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk890(t296: f64, t36007: f64, t1901: f64, t193: f64, t34108: f64, t36161: f64, t36165: f64, t36168: f64, t36172: f64, t36175: f64, t36179: f64, t36183: f64, t36188: f64, t36191: f64, t36195: f64, t446: f64, t89: f64) -> (f64, f64) {
    let t36199 = t296 * t36007;
    let t36202 = 2.0_f64 / 9.0_f64 * t1901 * t36161 - 2.0_f64 / 9.0_f64 * t1901 * t36165 - t446 * t36168 / 3.0_f64 - t446 * t36172 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t36175 - 2.0_f64 / 3.0_f64 * t446 * t36179 - 2.0_f64 / 9.0_f64 * t1901 * t36183 - t34108 + t446 * t36188 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t36191 + t89 * t193 * t36195 / 3.0_f64 - t446 * t36199 / 3.0_f64;
    (t36199, t36202)
}
