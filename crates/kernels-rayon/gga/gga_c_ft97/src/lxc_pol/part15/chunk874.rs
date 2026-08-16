//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 874/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk874(t41446: f64, t9651: f64, t2440: f64, t9570: f64, t703: f64, t9577: f64, t209: f64, t3626: f64, t228: f64, t231: f64, t191: f64, t2347: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41447 = t9651 * t41446;
    let t41458 = t2440 * t9570;
    let t41477 = t703 * t9577;
    let t41510 = t209 * t3626;
    let t41512 = t228 * t41510 * t231;
    let t41513 = 0.18916624705075445817e-1_f64 * t41512;
    let t41534 = 1.0_f64 / t191 / t2347;
    (t41447, t41458, t41477, t41512, t41513, t41534)
}
