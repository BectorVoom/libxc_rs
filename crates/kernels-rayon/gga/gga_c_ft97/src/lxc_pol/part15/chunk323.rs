//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 323/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk323(t137: f64, t548: f64, t135: f64, t1730: f64, t152: f64, t153: f64, t151: f64, t1771: f64, t143: f64, t1554: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2057 = 1.0_f64 / t548 / t137;
    let t2058 = t135 * t2057;
    let t2066 = 0.11113000182098765433e-1_f64 * t1730;
    let t2086 = 1.0_f64 / t153 / t152;
    let t2092 = 4.0_f64 / 9.0_f64 * t1771 * t151;
    let t2097 = t1554 * t143;
    (t2057, t2058, t2066, t2086, t2092, t2097)
}
