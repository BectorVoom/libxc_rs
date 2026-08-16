//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1414/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1414(t39521: f64, t41141: f64, t41150: f64, t41168: f64, t41174: f64, t41185: f64, t41191: f64, t41208: f64, t2985: f64, t3010: f64, t2988: f64) -> (f64, f64, f64) {
    let t41211 = t39521 + t41141 + t41150 + t41168 + t41174 + t41185 + t41191 + t41208;
    let t41224 = 1.0_f64 / t3010 / t2985;
    let t41225 = t2988 * t2988;
    (t41211, t41224, t41225)
}
