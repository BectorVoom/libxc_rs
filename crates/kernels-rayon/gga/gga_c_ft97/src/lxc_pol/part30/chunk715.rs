//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 715/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk715(t29097: f64, t29145: f64, t29197: f64, t29241: f64, t29284: f64, t29336: f64, t29374: f64, t29407: f64, t18986: f64, t2: f64, t4: f64, t26: f64) -> (f64, f64, f64) {
    let t29410 = t29097 + t29145 + t29197 + t29241 + t29284 + t29336 + t29374 + t29407;
    let t29414 = t18986 * t2;
    let t29415 = t29414 * t4;
    let t29416 = t29415 * t26;
    (t29410, t29414, t29416)
}
