//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 982/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk982(t12207: f64, t9823: f64, t41528: f64, t41532: f64, t41534: f64, t13846: f64, t1841: f64, t2536: f64, t734: f64, t2558: f64, t39002: f64, t9647: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47572 = t9823 * t12207;
    let t47574 = 0.38342925953920749677e0_f64 * t41528;
    let t47575 = 0.85206502119823888171e-1_f64 * t41532;
    let t47576 = 0.38342925953920749677e0_f64 * t41534;
    let t47587 = t1841 * t2536 * t13846 * t734;
    let t47594 = t9647 * t39002 * t2558;
    (t47572, t47574, t47575, t47576, t47587, t47594)
}
