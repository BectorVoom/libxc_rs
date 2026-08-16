//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 985/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk985(t39706: f64, t39763: f64, t40299: f64, t40514: f64, t605: f64, t2142: f64, t9258: f64, t604: f64, t9394: f64, t609: f64, t2133: f64, t2178: f64) -> (f64, f64, f64, f64) {
    let t40517 = t605 * (t39706 + t39763 + t40299 + t40514);
    let t40519 = t2142 * t9258;
    let t40521 = t9394 * t604;
    let t40522 = t40521 * t609;
    let t40524 = t2133 * t2178;
    (t40517, t40519, t40522, t40524)
}
