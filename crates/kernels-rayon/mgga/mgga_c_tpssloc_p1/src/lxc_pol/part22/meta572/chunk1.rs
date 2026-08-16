//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2081/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2081(t43776: f64, t2296: f64, t3241: f64, t11778: f64, t154: f64, t1091: f64, t9698: f64) -> (f64, f64, f64, f64) {
    let t43777 = 0.13490888888888888889e1_f64 * t43776;
    let t43791 = 1.0_f64 / t3241 / t2296;
    let t43809 = t154 * t11778;
    let t43816 = t9698 * t1091;
    (t43777, t43791, t43809, t43816)
}
