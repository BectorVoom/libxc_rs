//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 36/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk36(t69: f64, t72: f64, t68: f64, t66: f64, t64: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t73 = t69 * t72;
    let t74 = t68 * t73;
    let t76 = 1.0_f64 + 0.19153082513888888889e-1_f64 * t74;
    let t77 = 1.0_f64 / t76;
    let t78 = t66 * t77;
    let t79 = t64 * t78;
    let t80 = 0.1e-59_f64 < t79;
    let t81 = piecewise3(t80, t79, 0.1e-59_f64);
    (t74, t76, t77, t78, t81, t79)
}
