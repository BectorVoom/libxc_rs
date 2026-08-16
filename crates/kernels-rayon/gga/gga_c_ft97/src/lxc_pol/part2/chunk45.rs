//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 45/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk45(t2: f64, t82: f64, t24: f64, t92: f64, t91: f64, t85: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93 = t82 * t2;
    let t94 = t24 * t93;
    let t95 = t92 * t94;
    let t96 = f64::sqrt(t95);
    let t97 = t91 * t96;
    let t100 = 3.0_f64 + t97 / 3.0_f64 + t85 / 3.0_f64;
    (t93, t94, t95, t96, t97, t100)
}
