//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1079/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1079(t76087: f64, t76090: f64, t70387: f64, t76110: f64, t30204: f64, t78220: f64, t26291: f64, t78223: f64, t40724: f64, t78070: f64, t76113: f64, t76116: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t78486 = 0.5107751987195740728e-4_f64 * t76087;
    let t78487 = 0.2553875993597870364e-4_f64 * t76090;
    let t78488 = 0.38430329123504567781e-4_f64 * t70387;
    let t78491 = 0.14967802127329760705e-1_f64 * t76110;
    let t78493 = 0.23948483403727617128e0_f64 * t30204 * t78220;
    let t78495 = 0.35922725105591425692e0_f64 * t26291 * t78223;
    let t78497 = 0.35922725105591425692e0_f64 * t40724 * t78070;
    let t78498 = 0.44903406381989282115e-1_f64 * t76113;
    let t78499 = 0.2993560425465952141e-1_f64 * t76116;
    (t78486, t78487, t78488, t78491, t78493, t78495, t78497, t78498, t78499)
}
