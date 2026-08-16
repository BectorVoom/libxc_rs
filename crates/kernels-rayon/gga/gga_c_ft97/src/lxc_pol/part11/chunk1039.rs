//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1039/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1039(t41677: f64, t807: f64, t2426: f64, t2428: f64, t3724: f64, t41448: f64, t9577: f64, t683: f64, t92: f64, t41482: f64, t2360: f64, t41468: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41682 = t807 * t41677;
    let t41686 = t3724 * t2426 * t2428;
    let t41691 = t9577 * t41448;
    let t41693 = t92 * t683 * t41691;
    let t41696 = t92 * t683 * t41482;
    let t41698 = t2360 * t41468;
    (t41682, t41686, t41691, t41693, t41696, t41698)
}
