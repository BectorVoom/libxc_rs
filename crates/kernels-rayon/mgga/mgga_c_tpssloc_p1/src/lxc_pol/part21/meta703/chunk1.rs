//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2534/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2534(t13822: f64, t13881: f64, t973: f64, t13886: f64, t10263: f64, t4506: f64, t3082: f64, t4622: f64, t1040: f64, t13941: f64, t10231: f64, t13555: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48407 = t973 * t13822 * t13881;
    let t48417 = t973 * t13822 * t13886;
    let t48421 = t10263 * t4506;
    let t48430 = t4622 * t3082;
    let t48432 = t13941 * t1040;
    let t48441 = t973 * t10231 * t13555;
    (t48407, t48417, t48421, t48430, t48432, t48441)
}
