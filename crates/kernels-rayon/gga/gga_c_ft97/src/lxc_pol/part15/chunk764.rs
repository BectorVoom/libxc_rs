//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 764/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk764(t21222: f64, t224: f64, t695: f64, t5006: f64, t6762: f64, t1128: f64, t4986: f64, t206: f64, t5011: f64, t214: f64, t52: f64, t204: f64, t21210: f64, t41: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21224 = t224 * t695 * t21222;
    let t21225 = t6762 * t5006;
    let t21227 = t4986 * t1128;
    let t21233 = 1.0_f64 / t206 / t5011;
    let t21235 = t52 * t214 * t21233;
    let t21237 = -0.205601884870781893e1_f64 * t41 * t204 * t21210 - 0.13764695059989835716e0_f64 * t21235;
    (t21224, t21225, t21227, t21233, t21235, t21237)
}
