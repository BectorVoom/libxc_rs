//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1025/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1025(t457: f64, t86052: f64, t86102: f64, t86140: f64, t86168: f64, t91: f64, t446: f64, t447: f64, t85474: f64, t85456: f64, t38262: f64, t86090: f64) -> (f64, f64, f64, f64) {
    let t86172 = t91 * t457 * (t86052 + t86102 + t86140 + t86168);
    let t86175 = t446 * t447 * t85474;
    let t86178 = t446 * t447 * t85456;
    let t86181 = t446 * t38262 * t86090;
    (t86172, t86175, t86178, t86181)
}
