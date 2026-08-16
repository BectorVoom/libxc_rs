//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1222/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1222(t32507: f64, t32509: f64, t34431: f64, t34432: f64, t34433: f64, t37087: f64, t37090: f64, t39299: f64, t39302: f64, t39305: f64, t39308: f64, t39311: f64, t39314: f64, t39318: f64, t39320: f64, t39322: f64, t39324: f64, t39326: f64) -> f64 {
    let t41582 = -t37087 - 5.0_f64 / 16.0_f64 * t39299 - t39302 / 16.0_f64 + t39305 / 8.0_f64 + t39308 / 32.0_f64 + t39311 / 32.0_f64 - t39314 / 32.0_f64 - t32507 - t37090 - t34431 + t32509 + t34432 - 77.0_f64 / 432.0_f64 * t34433 + t39318 / 24.0_f64 - 0.16809375e0_f64 * t39320 + 0.16809375e0_f64 * t39322 + 0.3361875e0_f64 * t39324 - 11.0_f64 / 96.0_f64 * t39326;
    t41582
}
