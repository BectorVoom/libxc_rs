//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 579/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk579(t45: f64, t56: f64, t41: f64, t42: f64, t7936: f64, t78: f64, t388: f64, t391: f64, t625: f64, t68: f64, t72: f64, t2247: f64, t47: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8063 = 1.0_f64 / t45 / t56;
    let t8068 = -0.205601884870781893e1_f64 * t41 * t42 * t8063 - 0.13764695059989835716e0_f64 * t7936;
    let t8069 = t78 * t8068;
    let t8070 = t388 * t8069;
    let t8074 = t68 * t391 * t625 * t72;
    let t8076 = t47 * t2247;
    (t8063, t8068, t8069, t8070, t8074, t8076)
}
